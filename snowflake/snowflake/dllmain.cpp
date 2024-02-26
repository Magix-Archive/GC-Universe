#include <pch.h>

#include "app/main.h"

using namespace snowflake;

#pragma comment(lib, "detours.lib")
#pragma comment(lib, "ntdll.lib")

// ReSharper disable CppInconsistentNaming
BOOL APIENTRY DllMain(const HMODULE h_module,
                      const DWORD ul_reason_for_call
)
{
    if (h_module)
        DisableThreadLibraryCalls(h_module);

    if (ul_reason_for_call == DLL_PROCESS_ATTACH)
    {
        if (const auto hThread = CreateThread(nullptr, 0,
            (LPTHREAD_START_ROUTINE) snowflake_main,
            h_module, 0, nullptr))
            CloseHandle(hThread);
    }

    return TRUE;
}
