#include "pch.h"
#include "main.h"

#include "game.h"
#include "Console.h"
#include "Security.h"

using namespace utils;

void snowflake::snowflake_main(HMODULE h_module)
{
    create_console();

    wait_for_startup();
    LOG_DEBUG("Game has finished loading.");

    disable_logging();
    LOG_DEBUG("Logging has been disabled.");

    disable_memory_protections();
    LOG_DEBUG("Memory protections have been disabled.");

    LOG_INFO("Snowflake has finished loading.");
    LOG_INFO("Use 'help' to see a list of commands!");
}
