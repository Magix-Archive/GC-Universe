#pragma once

#include "utilities.hpp"

namespace snowflake
{
    inline bool show_console = false;
    inline bool log_debug = false;

    inline bool print_addresses = false;
    inline bool show_missing_keys = false;
    inline bool show_read_to_end = false;
    inline bool show_from_xml_string = false;

    /**
     * \brief Parses the configuration file, or creates it if it doesn't exist.
     */
    inline void parse()
    {
        // Check if the configuration file exists, if not, create it.
        if (!utils::file_exists("snowflake.ini"))
        {
            std::ofstream file("snowflake.ini");
            file << "[snowflake]\n";
            file << "ShowConsole = false\n";
            file << "LogDebug = false\n";
            file << "\n";
            file << "[debug]\n";
            file << "PrintAddresses = false\n";
            file << "ShowMissingKeys = false\n";
            file << "ReadToEnd = false\n";
            file << "FromXmlString = false\n";

            file.close();
        }

        inipp::Ini<char> ini;
        std::ifstream file("snowflake.ini");
        ini.parse(file);
        file.close();

        // Parse the configuration file.
        inipp::get_value(ini.sections["snowflake"], "ShowConsole", show_console);
        inipp::get_value(ini.sections["snowflake"], "LogDebug", log_debug);

        inipp::get_value(ini.sections["debug"], "PrintAddresses", print_addresses);
        inipp::get_value(ini.sections["debug"], "ShowMissingKeys", show_missing_keys);
        inipp::get_value(ini.sections["debug"], "ReadToEnd", show_read_to_end);
        inipp::get_value(ini.sections["debug"], "FromXmlString", show_from_xml_string);
    }
}
