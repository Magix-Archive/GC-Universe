#include <windows.h>
#include <TlHelp32.h>
#include <iostream>
#include <psapi.h>
#include <istream>
#include <tchar.h>
#include <string>

#define DRIVER_NAME "HoYoKProtect.sys"

typedef LONG(NTAPI* nt_suspend_process)(IN HANDLE process_handle);
typedef LONG(NTAPI* nt_resume_process)(IN HANDLE process_handle);

void print(const std::string& message)
{
    std::cout << message << std::endl;
}

bool DriverLoaded() 
{
    DWORD cb_needed;

    if (LPVOID drivers[1024]; EnumDeviceDrivers(drivers, sizeof(drivers), &cb_needed) && cb_needed < sizeof(drivers))
    {
        const int c_drivers = cb_needed / sizeof(drivers[0]);

        for (int i = 0; i < c_drivers; i++)
        {
            if (TCHAR sz_driver[1024]; GetDeviceDriverBaseName(drivers[i], sz_driver, std::size(sz_driver)))
            {
                if (const auto target_driver_name = TEXT("HoYoKProtect.sys");
                    _tcsicmp(sz_driver, target_driver_name) == 0)
                {
                    return true;
                }
            }
        }
    }

    return false;
}

DWORD FindProcessId(const std::string& processName)
{
    DWORD pid = -1;

    const HANDLE snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    PROCESSENTRY32 process;
    ZeroMemory(&process, sizeof(process));
    process.dwSize = sizeof(process);

    if (Process32Next(snapshot, &process))
    {
        do
        {
            if (std::string(process.szExeFile) == processName)
            {
                pid = process.th32ProcessID;
                break;
            }
        } while (Process32Next(snapshot, &process));
    }

    CloseHandle(snapshot);

    return pid;
}

bool OpenGame(const char* gamePath, HANDLE* phProcess, HANDLE* phThread)
{
    HANDLE hToken;
    if (!OpenProcessToken(GetCurrentProcess(), TOKEN_ALL_ACCESS, &hToken))
    {
        print("[Launch Game] Unable to escalate privileges.");
        return false;
    }

    const DWORD pidExplorer = FindProcessId("explorer.exe");
    if (pidExplorer == 0)
    {
        print("[Launch Game] Unable to find the 'explorer.exe' process ID.");
        return false;
    }

    HANDLE hExplorer = OpenProcess(PROCESS_ALL_ACCESS, false, pidExplorer);

    size_t size = 0;
    InitializeProcThreadAttributeList(nullptr, 1, 0, &size);

    const auto temp = new char[size];
    const auto attributeList = reinterpret_cast<PPROC_THREAD_ATTRIBUTE_LIST>(temp);
    InitializeProcThreadAttributeList(attributeList, 1, 0, &size);
    if (!UpdateProcThreadAttribute(
        attributeList, 0, PROC_THREAD_ATTRIBUTE_PARENT_PROCESS,
        &hExplorer, sizeof(HANDLE), nullptr, nullptr))
    {
        print("[Launch Game] Unable to update the process thread attribute list.");
    }

    auto processInfo = PROCESS_INFORMATION{};
    auto startInfo = STARTUPINFOEXA{};
    startInfo.StartupInfo.cb = sizeof(startInfo);
    startInfo.lpAttributeList = attributeList;

    const auto result = CreateProcessAsUserA(
        hToken, gamePath, nullptr, nullptr, nullptr,
        false, EXTENDED_STARTUPINFO_PRESENT, nullptr,
        nullptr, (LPSTARTUPINFOA) &startInfo, &processInfo);

    DeleteProcThreadAttributeList(attributeList);
    delete[] temp;

    if (result)
    {
        *phThread = processInfo.hThread;
        *phProcess = processInfo.hProcess;
    }
    else
    {
        print("[Launch Game] Unable to create the process.");
    }

    return result;
}

void InternalSuspend(const HANDLE* phProcess)
{
    if (const auto pfn_nt_suspend_process = reinterpret_cast<nt_suspend_process>(GetProcAddress(
        GetModuleHandle(TEXT("ntdll")), "NtSuspendProcess")); pfn_nt_suspend_process != nullptr)
    {
        pfn_nt_suspend_process(*phProcess);
        print("[Disable AC] Process suspended successfully.");
    }
    else
    {
        print("[Disable AC] Unable to fetch 'NtSuspendProcess'.");
    }
}

void InternalResume(const HANDLE* phProcess)
{
    if (const auto pfn_nt_resume_process = reinterpret_cast<nt_resume_process>(GetProcAddress(
        GetModuleHandle(TEXT("ntdll")), "NtResumeProcess")); pfn_nt_resume_process != nullptr)
    {
        pfn_nt_resume_process(*phProcess);
        print("[Disable AC] Process resumed successfully.");
    }
    else
    {
        print("[Disable AC] Unable to fetch 'NtResumeProcess'.");
    }
}

void WaitForDriver(const HANDLE* phProcess)
{
    print("Searching for 'HoYoKProtect.sys'...");

    DWORD needed;
    auto driverFound = false;
    while (!driverFound)
    {
        LPVOID drivers[1024];
        if (!EnumDeviceDrivers(drivers, sizeof(drivers), &needed) || needed > sizeof(drivers))
        {
            print("Unable to fetch drivers. Overallocation with driver list.");
            break;
        }

        const int driverCount = needed / sizeof(drivers[0]);

        for (auto i = 0; i < driverCount; i++)
        {
            TCHAR driverName[1024];
            if (!GetDeviceDriverBaseName(drivers[i], driverName, std::size(driverName)))
            {
                print("Unable to fetch drivers. Cannot fetch name of driver.");
                continue;
            }

            if (_tcsicmp(driverName, DRIVER_NAME) != 0) continue;
            print("Driver detected at " + std::to_string((uintptr_t)drivers[i]));

            driverFound = true;

            InternalSuspend(phProcess);

            while (DriverLoaded())
            {
                Sleep(100);
            }

            InternalResume(phProcess);
        }
    }
}

void InjectDll(const HANDLE hProcess, const std::string& dllPath)
{
    const HMODULE hKernel = GetModuleHandle("kernel32.dll");
    if (hKernel == nullptr)
    {
        print("[DLL Injection] Failed to get kernel32.dll module address.");
        return;
    }

    const auto pLoadLibrary = (LPVOID)GetProcAddress(hKernel, "LoadLibraryA");
    if (pLoadLibrary == nullptr) {
        print("[DLL Injection] Failed to get LoadLibraryA address.");
        return;
    }

    const LPVOID pDLLPath = VirtualAllocEx(hProcess, nullptr, strlen(dllPath.c_str()) + 1, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    if (pDLLPath == nullptr) {
        print("[DLL Injection] Failed to allocate memory for DLLPath in target process.");
        return;
    }

    // Write the string name of our DLL in the memory allocated
    if (const bool writeResult = WriteProcessMemory(
        hProcess, pDLLPath, dllPath.c_str(),
        strlen(dllPath.c_str()), nullptr); writeResult == FALSE) {
        print("[DLL Injection] Failed to write remote process memory.");
        return;
    }

    // Load our DLL by calling LoadLibrary in the other process and passing our dll name
    const HANDLE hThread = CreateRemoteThread(
        hProcess, nullptr, NULL,
        (LPTHREAD_START_ROUTINE)pLoadLibrary, pDLLPath,
        NULL, nullptr);
    if (hThread == nullptr) {
        print("[DLL Injection] Failed to create remote thread.");
        VirtualFreeEx(hProcess, pDLLPath, 0, MEM_RELEASE);
        return;
    }

    // Waiting for thread end and release unnecessary data.
    if (WaitForSingleObject(hThread, 2000) == WAIT_OBJECT_0)
    {
        VirtualFreeEx(hProcess, pDLLPath, 0, MEM_RELEASE);
    }

    CloseHandle(hThread);

    print("[DLL Injection] Successfully performed LoadLibraryA injection.");
    return;
}

extern "C" void open_game(const char* game_path, const char* dll_path) {
    print("[Launch Game] Opening game from " + std::string(game_path) + "...");

    HANDLE hProcess, hThread;
    if (!OpenGame(game_path, &hProcess, &hThread))
    {
        print("[Launch Game] Unable to open the game.");
        return;
    }

    WaitForDriver(&hProcess);

    print("[DLL Injection] Waiting for the game to load...");
    Sleep(5e3);

    print("[DLL Injection] Injecting DLL from " + std::string(dll_path) + "...");

    InjectDll(hProcess, std::string(dll_path));

    Sleep(2e3);
    ResumeThread(hThread);
    CloseHandle(hProcess);
}
