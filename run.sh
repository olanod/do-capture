#!/bin/sh
firefox -headless -marionette & 
FIREFOX_PID=$!
geckodriver --connect-existing --marionette-port 2828 &
GECKO_PID=$!

capture $1

kill $GECKO_PID
kill $FIREFOX_PID
