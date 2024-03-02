#pragma once
#include "utilities.hpp"

namespace snowflake
{
    inline void* o_read_to_end = nullptr;

    inline String* read_to_end(void* rcx, void* rdx)
    {
        auto result = decltype(&read_to_end)(o_read_to_end)(rcx, rdx);
        if (!result) return result;

        LOG_DEBUG("called read_to_end");

        return result;
    }

    inline void patch_all()
    {
        // Hook ReadToEnd function.
        const auto p_read_to_end = utils::pattern_scan(
            "UserAssembly.dll",
            "48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC 20 48 83 79 ? ? 48 8B D9 75 05");
        LOG_DEBUG("ReadToEnd is at 0x" + std::to_string(p_read_to_end));
        if (!p_read_to_end || p_read_to_end % 16 > 0)
        {
            LOG_ERROR("Failed to find 'ReadToEnd'!");
        }
        else
        {
            o_read_to_end = detour((void*) p_read_to_end, read_to_end, true);
            LOG_DEBUG("Hooked 'ReadToEnd' at 0x" + std::to_string((uintptr_t) o_read_to_end));
        }
    }
}
