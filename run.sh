#!/bin/sh
firefox -headless -marionette & 
FIREFOX_PID=$!
geckodriver --connect-existing --marionette-port 2828 &
GECKO_PID=$!

capture "$@"

kill $GECKO_PID
kill $FIREFOX_PID
