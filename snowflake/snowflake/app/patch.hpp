#pragma once

#include <comdef.h>

#include "config.h"
#include "utilities.hpp"

const LPCSTR public_key = "<RSAKeyValue><Modulus>xbbx2m1feHyrQ7jP+8mtDF/pyYLrJWKWAdEv3wZrOtjOZzeLGPzsmkcgncgoRhX4dT+1itSMR9j9m0/OwsH2UoF6U32LxCOQWQD1AMgIZjAkJeJvFTrtn8fMQ1701CkbaLTVIjRMlTw8kNXvNA/A9UatoiDmi4TFG6mrxTKZpIcTInvPEpkK2A7Qsp1E4skFK8jmysy7uRhMaYHtPTsBvxP0zn3lhKB3W+HTqpneewXWHjCDfL7Nbby91jbz5EKPZXWLuhXIvR1Cu4tiruorwXJxmXaP1HQZonytECNU/UOzP6GNLdq0eFDE4b04Wjp396551G99YiFP2nqHVJ5OMQ==</Modulus><Exponent>AQAB</Exponent></RSAKeyValue>";

namespace snowflake
{
    inline void* o_from_xml_string = nullptr;
    inline void* o_read_to_end = nullptr;

    inline void __fastcall from_xml_string(void* rcx, String* xmlString)
    {
        const auto string = xmlString->c_str();
        const _bstr_t xml_string(string);
        if (show_from_xml_string) LOG_DEBUG("[FromXmlString] String output: `" + std::string(xml_string) + "`");

        std::string new_key{};
        if (wcsstr(xmlString->c_str(), L"<InverseQ>"))
        {
            if (show_from_xml_string) LOG_DEBUG("[FromXmlString] Asking for private key!");
        }
        else
        {
            new_key = public_key;
        }

        if (!new_key.empty() && new_key.size() <= xmlString->size())
        {
            ZeroMemory(xmlString->chars, xmlString->size() * 2);
            const auto wide_key = std::wstring(new_key.begin(), new_key.end());
            memcpy_s(xmlString->chars, xmlString->size() * 2, wide_key.data(), wide_key.size() * 2);
        }

        (decltype(&from_xml_string)(o_from_xml_string)(rcx, xmlString));
    }

    inline String* __fastcall read_to_end(void* rcx, void* rdx)
    {
        const auto result = decltype(&read_to_end)(o_read_to_end)(rcx, rdx);
        if (!result) return result;

        const auto string = result->c_str();
        const _bstr_t xml_string(string);
        if (show_read_to_end) LOG_DEBUG("[ReadToEnd] String output: `" + std::string(xml_string) + "`");

        if (!wcsstr(result->c_str(), L"<RSAKeyValue>")) return result;

        std::string new_key{};
        if (wcsstr(result->c_str(), L"<InverseQ>"))
        {
            if (show_read_to_end) LOG_DEBUG("[ReadToEnd] Asking for private key!");
        }
        else
        {
            new_key = public_key;
        }

        if (!new_key.empty() && new_key.size() <= result->size())
        {
            ZeroMemory(result->chars, result->size() * 2);
            const auto wide_key = std::wstring(new_key.begin(), new_key.end()); // idc
            memcpy_s(result->chars, result->size() * 2, wide_key.data(), wide_key.size() * 2);
        }

        return result;
    }

    inline void patch_all()
    {
        // Hook FromXmlString function.
        const auto p_from_xml_string = utils::pattern_scan(
            "UserAssembly.dll",
            "48 8B C4 48 89 58 10 48 89 78 18 4C 89 70 20 55 48 8D 68 A1 48 81 EC A0 00 00 00 48 8B FA 4C 8B F1");
        if (print_addresses) LOG_DEBUG("FromXmlString is at 0x" + std::to_string(p_from_xml_string));
        if (!p_from_xml_string || p_from_xml_string % 16 > 0)
        {
            LOG_ERROR("Failed to find 'FromXmlString'!");
        }
        else
        {
            o_from_xml_string = detour((void*) p_from_xml_string, (void*) from_xml_string, true);
            if (print_addresses) LOG_DEBUG("Hooked 'FromXmlString' at 0x" + std::to_string((uintptr_t) o_from_xml_string));
        }

        // Hook ReadToEnd function.
        const auto p_read_to_end = utils::pattern_scan(
            "UserAssembly.dll",
            "48 89 5C 24 ? 48 89 74 24 ? 48 89 7C 24 ? 41 56 48 83 EC 20 48 83 79 ? ? 48 8B D9 75 05");
        if (print_addresses) LOG_DEBUG("ReadToEnd is at 0x" + std::to_string(p_read_to_end));
        if (!p_read_to_end || p_read_to_end % 16 > 0)
        {
            LOG_ERROR("Failed to find 'ReadToEnd'!");
        }
        else
        {
            o_read_to_end = detour((void*) p_read_to_end, (void*) read_to_end, true);

            if (print_addresses) LOG_DEBUG("Hooked 'ReadToEnd' at 0x" + std::to_string((uintptr_t) o_read_to_end));
        }
    }
}
