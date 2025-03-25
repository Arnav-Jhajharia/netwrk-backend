@echo off
set num_requests=200

for /L %%i in (1, 1, %num_requests%) do (
    start /b curl -X POST -H "Authentication: adi" -H "Content-Type: application/avro-binary" --data-binary "@C:/Users/Aditya/Downloads/userdata1.avro" "https://127.0.0.1:8443/streams" --insecure  > NUL 2>&1
)

timeout /t -1