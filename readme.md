What is amqp? 
Jawab:
AMQP (Advanced Message Queuing Protocol) adalah protokol standar yang digunakan oleh message broker (seperti RabbitMQ) untuk menerima, menyimpan, dan meneruskan pesan antar sistem.

What does guest:guest@localhost:5672 mean? 
Jawab:
Itu adalah URL koneksi ke RabbitMQ.
guest (pertama) = Username default RabbitMQ.
guest (kedua) = Password default RabbitMQ.
localhost = Alamat server (karena RabbitMQ nanti akan dijalankan di laptopmu sendiri via Docker).
5672 = Port standar yang digunakan untuk protokol AMQP.

![alt text](<Screenshot 2026-05-12 204832.png>) ![alt text](<Screenshot 2026-05-12 204907.png>) ![alt text](<Screenshot 2026-05-12 204912.png>) ![alt text](<Screenshot 2026-05-12 204935.png>) ![alt text](<Screenshot 2026-05-12 205001.png>)

Mengapa antrian di RabbitMQ turun lebih cepat saat ada 3 subscriber?

Karena beban kerja dibagi secara Round Robin ke beberapa subscriber, sehingga total throughput sistem meningkat

![alt text](image.png)