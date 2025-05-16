#!/bin/sh

if [ ! -f /config/settings.poti ]; then
    echo "No config found in /config"
    touch /config/settings.poti
fi

exec /app/poti
