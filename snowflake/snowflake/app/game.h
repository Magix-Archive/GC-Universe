#pragma once

namespace snowflake
{
    inline void wait_for_startup()
    {
        auto pid = GetCurrentProcessId();
        while (true)
        {
            // use EnumWindows to pinpoint the target window
            // as there could be other window with the same class name
            EnumWindows([](const HWND hwnd, const LPARAM lParam)->BOOL __stdcall {

                DWORD windowProcessId = 0;
                GetWindowThreadProcessId(hwnd, &windowProcessId);

                char szWindowClass[256]{};
                GetClassNameA(hwnd, szWindowClass, 256);
                if (!strcmp(szWindowClass, "UnityWndClass") && windowProcessId == *(DWORD*)lParam)
                {
                    *(DWORD*)lParam = 0;
                    return FALSE;
                }

                return TRUE;

                }, (LPARAM)&pid);

            if (!pid)
                break;

            Sleep(2000);
            LOG_DEBUG("Waiting 2 seconds for the game to load...");
        }

        if (auto module_base = GetModuleHandleW(L"UserAssembly.dll"))
        {
            LOG_DEBUG("UserAssembly.dll is at 0x" + std::to_string((uintptr_t) module_base));
            LOG_INFO("Waiting 15 seconds for game library to load...");
            Sleep(15000);
        }
    }
}
