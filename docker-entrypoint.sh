#!/bin/bash
set -e

echo "========================================="
echo "POLIMARKET API - Container Starting"
echo "========================================="
echo ""

# Print environment info for debugging
echo "🔍 Environment Information:"
echo "  - User: $(whoami)"
echo "  - Working Directory: $(pwd)"
echo "  - Binary exists: $(test -f ./poli_market_api && echo 'YES' || echo 'NO')"
echo "  - Binary is executable: $(test -x ./poli_market_api && echo 'YES' || echo 'NO')"
echo ""

# Check critical environment variables
echo "🔍 Environment Variables:"
echo "  - RUST_LOG: ${RUST_LOG:-'not set (will default to info)'}"
echo "  - SERVER_HOST: ${SERVER_HOST:-'not set (will default to 0.0.0.0)'}"
echo "  - SERVER_PORT: ${SERVER_PORT:-'not set (will default to 8080)'}"

if [ -z "$DATABASE_URL" ]; then
    echo "  - DATABASE_URL: ❌ NOT SET - THIS WILL CAUSE FAILURE!"
    echo ""
    echo "❌ ERROR: DATABASE_URL environment variable is required!"
    echo "   Please set it in your Dokploy configuration."
    echo "   Format: postgres://user:password@host:5432/database"
    echo ""
    exit 1
else
    # Mask password in log
    MASKED_URL=$(echo "$DATABASE_URL" | sed -E 's/(:[^:@]+)@/:*****@/')
    echo "  - DATABASE_URL: ✅ SET ($MASKED_URL)"
fi

echo ""
echo "========================================="
echo "🚀 Starting PoliMarket API..."
echo "========================================="
echo ""

# Execute the application
exec "$@"
