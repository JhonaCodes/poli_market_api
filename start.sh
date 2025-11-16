#!/bin/bash
set -e

echo "========================================"
echo "  POLIMARKET API - STARTUP"
echo "========================================"
echo ""

# Environment check
echo "[CHECK] Environment variables:"
if [ -z "$DATABASE_URL" ]; then
    echo "[ERROR] DATABASE_URL is not set!"
    exit 1
fi
echo "[OK] DATABASE_URL is configured"
echo ""

# Database connection test
echo "[CHECK] Testing database connection..."
DB_HOST=$(echo $DATABASE_URL | sed -n 's/.*@\([^:]*\):.*/\1/p')
DB_PORT=$(echo $DATABASE_URL | sed -n 's/.*:\([0-9]*\)\/.*/\1/p')
DB_USER=$(echo $DATABASE_URL | sed -n 's/.*:\/\/\([^:]*\):.*/\1/p')

MAX_ATTEMPTS=30
ATTEMPT=1

while [ $ATTEMPT -le $MAX_ATTEMPTS ]; do
    if pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" > /dev/null 2>&1; then
        echo "[OK] Database is ready!"
        break
    else
        if [ $ATTEMPT -eq $MAX_ATTEMPTS ]; then
            echo "[ERROR] Database not accessible after $MAX_ATTEMPTS attempts"
            exit 1
        fi
        echo "[WAIT] Waiting for database... ($ATTEMPT/$MAX_ATTEMPTS)"
        sleep 1
        ATTEMPT=$((ATTEMPT + 1))
    fi
done
echo ""

# Binary check
echo "[CHECK] Application binary..."
if [ ! -x "./poli_market_api" ]; then
    echo "[ERROR] Binary not found or not executable"
    exit 1
fi
echo "[OK] Binary ready"
echo ""

echo "========================================"
echo "  STARTING POLIMARKET API"
echo "========================================"
echo "  Port: ${SERVER_PORT:-8080}"
echo "  Log: ${RUST_LOG:-info}"
echo "========================================"
echo ""

# Start application
exec ./poli_market_api
