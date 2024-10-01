echo "build postgres"
docker compose -f dev.yml -p vr360action up -d --build
