use std::io;

// Struct untuk film
struct Film {
    id: u32,
    judul: String,
    harga: u32,
}

// Struct untuk pesanan
struct Pesanan {
    judul: String,
    jumlah_tiket: u32,
    total_harga: u32,
}

fn main() {
    let daftar_film = vec![
        Film {
            id: 1,
            judul: "Spider-Man: No Way Home".to_string(),
            harga: 100_000,
        },
        Film {
            id: 2,
            judul: "The Batman".to_string(),
            harga: 120_000,
        },
        Film {
            id: 3,
            judul: "Avatar: The Way of Water".to_string(),
            harga: 150_000,
        },
        Film {
            id :4,
            judul:"Interstellar".to_string(),
            harga: 150_000,
        },
        Film {
            id:5,
            judul:"The Amazing Spiderman".to_string(),
            harga: 150_000,
        },
        Film {
            id: 6,
            judul:"Oppenheimaer".to_string(),
            harga: 150_000,
        },
        Film {
            id :7,
            judul:"The Batman".to_string(),
            harga:120_000,
        },
    ];

    let mut riwayat_pesanan: Vec<Pesanan> = Vec::new();

    loop {
        println!("\nSelamat datang di Sistem Pemesanan Tiket Bioskop!\n");
        println!("Daftar Film:");
        for film in &daftar_film {
            println!("{}. {} (Rp {})", film.id, film.judul, film.harga);
        }

        // Pilih film
        let film_id = input_u32("Masukkan ID film yang ingin Anda pesan: ");
        let film = daftar_film.iter().find(|&f| f.id == film_id);

        match film {
            Some(film) => {
                let jumlah_tiket = input_u32("Masukkan jumlah tiket: ");
                let total_harga = film.harga * jumlah_tiket;

                println!("\nDetail Pesanan:");
                println!("Film: {}", film.judul);
                println!("Jumlah Tiket: {}", jumlah_tiket);
                println!("Total Harga: Rp {}", total_harga);

                // Simpan pesanan ke riwayat
                riwayat_pesanan.push(Pesanan {
                    judul: film.judul.clone(),
                    jumlah_tiket,
                    total_harga,
                });

                // Tanya apakah ingin memesan lagi
                let lanjut = input_string("Apakah Anda ingin memesan lagi? (y/n): ");
                if lanjut.trim().to_lowercase() == "n" {
                    break;
                }
            }
            None => println!("ID film tidak valid. Silakan coba lagi."),
        }
    }

    // Tampilkan riwayat pesanan
    println!("\nRiwayat Pesanan:");
    for (i, pesanan) in riwayat_pesanan.iter().enumerate() {
        println!(
            "{}. {}, {} tiket, Total: Rp {}",
            i + 1,
            pesanan.judul,
            pesanan.jumlah_tiket,
            pesanan.total_harga
        );
    }

    println!("\nTerima kasih telah menggunakan sistem kami!");
}

fn input_u32(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Input tidak valid. Masukkan angka."),
        }
    }
}

fn input_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    input
}
