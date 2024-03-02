#pragma once

typedef enum _SECTION_INFORMATION_CLASS {
    SectionBasicInformation,
    SectionImageInformation
} SECTION_INFORMATION_CLASS, * PSECTION_INFORMATION_CLASS;
EXTERN_C NTSTATUS __stdcall NtQuerySection(HANDLE SectionHandle, SECTION_INFORMATION_CLASS InformationClass, PVOID InformationBuffer, ULONG InformationBufferSize, PULONG ResultLength);
EXTERN_C NTSTATUS __stdcall NtProtectVirtualMemory(HANDLE ProcessHandle, PVOID* BaseAddress, PULONG  NumberOfBytesToProtect, ULONG NewAccessProtection, PULONG OldAccessProtection);
EXTERN_C NTSTATUS __stdcall NtPulseEvent(HANDLE EventHandle, PULONG PreviousState);

namespace utils
{
    inline void disable_logging()
    {
        char szProcessPath[MAX_PATH]{};
        GetModuleFileNameA(nullptr, szProcessPath, MAX_PATH);

        const auto path = std::filesystem::path(szProcessPath);
        auto ProcessName = path.filename().string();
        ProcessName = ProcessName.substr(0, ProcessName.find_last_of('.'));

        const auto Astrolabe = path.parent_path() / (ProcessName + "_Data\\Plugins\\Astrolabe.dll");
        const auto MiHoYoMTRSDK = path.parent_path() / (ProcessName + "_Data\\Plugins\\MiHoYoMTRSDK.dll");

        HANDLE hFile = CreateFileA(Astrolabe.string().c_str(), GENERIC_READ | GENERIC_WRITE, 0, nullptr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);
        hFile = CreateFileA(MiHoYoMTRSDK.string().c_str(), GENERIC_READ | GENERIC_WRITE, 0, nullptr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);
    }

    inline void disable_memory_protections()
    {
        const auto ntdll = GetModuleHandleA("ntdll.dll");
        if (ntdll == nullptr) return;

        const bool linux = GetProcAddress(ntdll, "wine_get_version") != nullptr;
        void* routine = linux ? (void*)NtPulseEvent : (void*)NtQuerySection;
        DWORD old;
        VirtualProtect(NtProtectVirtualMemory, 1, PAGE_EXECUTE_READWRITE, &old);
        *(uintptr_t*)NtProtectVirtualMemory = *(uintptr_t*)routine & ~(0xFFui64 << 32) | (uintptr_t)(*(uint32_t*)((uintptr_t)routine + 4) - 1) << 32;
        VirtualProtect(NtProtectVirtualMemory, 1, old, &old);
    }
}
