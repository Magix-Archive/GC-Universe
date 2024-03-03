#pragma once

#include <comdef.h>
#include "utilities.hpp"

const LPCSTR public_key = "<RSAKeyValue><Modulus>xbbx2m1feHyrQ7jP+8mtDF/pyYLrJWKWAdEv3wZrOtjOZzeLGPzsmkcgncgoRhX4dT+1itSMR9j9m0/OwsH2UoF6U32LxCOQWQD1AMgIZjAkJeJvFTrtn8fMQ1701CkbaLTVIjRMlTw8kNXvNA/A9UatoiDmi4TFG6mrxTKZpIcTInvPEpkK2A7Qsp1E4skFK8jmysy7uRhMaYHtPTsBvxP0zn3lhKB3W+HTqpneewXWHjCDfL7Nbby91jbz5EKPZXWLuhXIvR1Cu4tiruorwXJxmXaP1HQZonytECNU/UOzP6GNLdq0eFDE4b04Wjp396551G99YiFP2nqHVJ5OMQ==</Modulus><Exponent>AQAB</Exponent></RSAKeyValue>";

namespace snowflake
{
    inline void* o_from_xml_string = nullptr;

    inline void __fastcall from_xml_string(void* rcx, String* xmlString)
    {
        LOG_DEBUG("called 'from_xml_string'!");
        const auto string = xmlString->c_str();
        const _bstr_t xml_string(string);
        LOG_DEBUG("string output: " + std::string(xml_string) + "!");

        std::string new_key{};
        const auto is_private = wcsstr(xmlString->c_str(), L"<InverseQ>");

        if (is_private)
        {
            LOG_DEBUG("Asking for private key!");
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

    inline void patch_all()
    {
        // Hook FromXmlString function.
        const auto p_from_xml_string = utils::pattern_scan(
            "UserAssembly.dll",
            "48 8B C4 48 89 58 10 48 89 78 18 4C 89 70 20 55 48 8D 68 A1 48 81 EC A0 00 00 00 48 8B FA 4C 8B F1");
        LOG_DEBUG("FromXmlString is at 0x" + std::to_string(p_from_xml_string));
        if (!p_from_xml_string || p_from_xml_string % 16 > 0)
        {
            LOG_ERROR("Failed to find 'FromXmlString'!");
        }
        else
        {
            o_from_xml_string = detour((void*) p_from_xml_string, (void*) from_xml_string, true);
            LOG_DEBUG("Hooked 'FromXmlString' at 0x" + std::to_string((uintptr_t) o_from_xml_string));
        }
    }
}
