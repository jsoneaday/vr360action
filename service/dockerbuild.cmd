echo "build postgres"
docker compose -f dev.yml -p vr360action up -d --build
sqlx migrate run --database-url postgres://vr360action:vr360action@localhost:5432/vr360action