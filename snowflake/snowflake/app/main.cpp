#include "pch.h"
#include "main.h"

#include "config.h"
#include "game.h"
#include "Console.h"
#include "patch.hpp"
#include "security.h"

using namespace utils;

void snowflake::snowflake_main(HMODULE h_module)
{
    // Load the configuration file.
    parse();

    if (show_console)
    {
        create_console();
    }

    wait_for_startup();
    LOG_DEBUG("Game has finished loading.");

    disable_logging();
    LOG_DEBUG("Logging has been disabled.");

    disable_memory_protections();
    LOG_DEBUG("Memory protections have been disabled.");

    patch_all();

    LOG_INFO("Snowflake has finished loading.");
    LOG_INFO("Use 'help' to see a list of commands!");
}
