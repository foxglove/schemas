import foxglove
import logging

# Make the SDK logging optionally available to the python client using verbose_on() or verbose_off()
# The log level can also be set with verbose_on(level) (default is "debug")

print("==== Verbose ON (default) ====")

foxglove.verbose_on()
# Can also specify log level with:
foxglove.verbose_on("warn")  # "debug" | "info" | "warn" | "error"
foxglove.verbose_on(logging.INFO)  # DEBUG | INFO | WARNING | ERROR

server = foxglove.start_server("info_tracing", port=8765)
# TODO: there is no longer a mechanism to stop the server manually
# server.close()

print("==== Verbose OFF ====")

foxglove.verbose_off()

server = foxglove.start_server("no_tracing", port=8766)
# server.close()
