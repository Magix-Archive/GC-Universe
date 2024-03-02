#pragma once
#include <pch.h>

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

    /**
     * \brief Detour a function to another function.
     * \param p_target Pointer to the target to detour.
     * \param p_detour Pointer to the detour function.
     * \param attach Whether to attach or detach the detour.
     * \return A pointer to the original function.
     */
    void* detour(void* p_target, void* p_detour, bool attach)
    {
        if (!p_target) return nullptr;

        auto originalFunc = p_target;

        DetourTransactionBegin();
        DetourUpdateThread((HANDLE) -2);

        if (attach)
        {
            DetourAttach(&originalFunc, p_detour);
        }
        else
        {
            DetourDetach(&originalFunc, p_detour);
        }
        DetourTransactionCommit();

        return originalFunc;
    }
}
