use std::io::{self, Write};

#[derive(Debug)]
struct Banka {
    id: u32,
    isim: String,
    hesaplar: Vec<u32>,
}

#[derive(Debug)]
struct Hesap {
    id: u32,
    bakiye: f64,
    banka_id: u32,
    sahipler: Vec<u32>,
}

#[derive(Debug)]
struct Kullanıcı {
    id: u32,
    isim: String,
    hesaplar: Vec<u32>,
}

#[derive(Debug)]
struct Paraİsteği {
    gönderen_hesap: u32,
    alıcı_hesap: u32,
    miktar: f64,
    durum: String,
}

struct BankacılıkSistemi {
    bankalar: Vec<Banka>,
    hesaplar: Vec<Hesap>,
    kullanıcılar: Vec<Kullanıcı>,
    istekler: Vec<Paraİsteği>,
}

impl BankacılıkSistemi {
    fn yeni() -> Self {
        BankacılıkSistemi {
            bankalar: Vec::new(),
            hesaplar: Vec::new(),
            kullanıcılar: Vec::new(),
            istekler: Vec::new(),
        }
    }

    fn banka_oluştur(&mut self, id: u32, isim: String) {
        self.bankalar.push(Banka {
            id,
            isim,
            hesaplar: Vec::new(),
        });
    }

    fn kullanıcı_oluştur(&mut self, id: u32, isim: String) {
        self.kullanıcılar.push(Kullanıcı {
            id,
            isim,
            hesaplar: Vec::new(),
        });
    }

    fn hesap_oluştur(&mut self, hesap_id: u32, banka_id: u32, kullanıcı_id: u32, başlangıç_bakiyesi: f64) -> Result<(), String> {
        if !self.bankalar.iter().any(|b| b.id == banka_id) {
            return Err("Banka bulunamadı".to_string());
        }
        if !self.kullanıcılar.iter().any(|u| u.id == kullanıcı_id) {
            return Err("Kullanıcı bulunamadı".to_string());
        }

        let hesap = Hesap {
            id: hesap_id,
            bakiye: başlangıç_bakiyesi,
            banka_id,
            sahipler: vec![kullanıcı_id],
        };

        if let Some(banka) = self.bankalar.iter_mut().find(|b| b.id == banka_id) {
            banka.hesaplar.push(hesap_id);
        }
        if let Some(kullanıcı) = self.kullanıcılar.iter_mut().find(|u| u.id == kullanıcı_id) {
            kullanıcı.hesaplar.push(hesap_id);
        }

        self.hesaplar.push(hesap);
        Ok(())
    }

    fn para_transfer_et(&mut self, gönderen_hesap_id: u32, alıcı_hesap_id: u32, miktar: f64) -> Result<(), String> {
        let gönderen_indeks = self.hesaplar.iter().position(|a| a.id == gönderen_hesap_id);
        let alıcı_indeks = self.hesaplar.iter().position(|a| a.id == alıcı_hesap_id);

        match (gönderen_indeks, alıcı_indeks) {
            (Some(gönderen_idx), Some(alıcı_idx)) if gönderen_idx != alıcı_idx => {
                let (gönderen, alıcı) = if gönderen_idx < alıcı_idx {
                    let (sol, sağ) = self.hesaplar.split_at_mut(alıcı_idx);
                    (&mut sol[gönderen_idx], &mut sağ[0])
                } else {
                    let (sol, sağ) = self.hesaplar.split_at_mut(gönderen_idx);
                    (&mut sağ[0], &mut sol[alıcı_idx])
                };

                if gönderen.bakiye < miktar {
                    return Err("Yetersiz bakiye".to_string());
                }
                if miktar <= 0.0 {
                    return Err("Miktar pozitif olmalı".to_string());
                }

                gönderen.bakiye -= miktar;
                alıcı.bakiye += miktar;
                Ok(())
            }
            (Some(_), Some(_)) => Err("Aynı hesaba transfer yapılamaz".to_string()),
            _ => Err("Hesaplardan biri veya her ikisi bulunamadı".to_string()),
        }
    }

    fn para_iste(&mut self, gönderen_hesap_id: u32, alıcı_hesap_id: u32, miktar: f64) -> Result<(), String> {
        if !self.hesaplar.iter().any(|a| a.id == gönderen_hesap_id) || !self.hesaplar.iter().any(|a| a.id == alıcı_hesap_id) {
            return Err("Hesaplardan biri veya her ikisi bulunamadı".to_string());
        }
        if miktar <= 0.0 {
            return Err("Miktar pozitif olmalı".to_string());
        }

        let istek = Paraİsteği {
            gönderen_hesap: gönderen_hesap_id,
            alıcı_hesap: alıcı_hesap_id,
            miktar,
            durum: "Beklemede".to_string(),
        };
        self.istekler.push(istek);
        Ok(())
    }

    // Yeni fonksiyon: Hesaba para ekleme
    fn para_ekle(&mut self, hesap_id: u32, miktar: f64) -> Result<(), String> {
        if miktar <= 0.0 {
            return Err("Eklenen miktar pozitif olmalı".to_string());
        }

        match self.hesaplar.iter_mut().find(|a| a.id == hesap_id) {
            Some(hesap) => {
                hesap.bakiye += miktar;
                Ok(())
            }
            None => Err("Hesap bulunamadı".to_string()),
        }
    }

    // Yeni fonksiyon: Hesaptan para çekme
    fn para_çek(&mut self, hesap_id: u32, miktar: f64) -> Result<(), String> {
        if miktar <= 0.0 {
            return Err("Çekilen miktar pozitif olmalı".to_string());
        }

        match self.hesaplar.iter_mut().find(|a| a.id == hesap_id) {
            Some(hesap) => {
                if hesap.bakiye < miktar {
                    return Err("Yetersiz bakiye".to_string());
                }
                hesap.bakiye -= miktar;
                Ok(())
            }
            None => Err("Hesap bulunamadı".to_string()),
        }
    }

    fn durumu_görüntüle(&self) {
        println!("\nGeçerli durum:");
        println!("Bankalar: {:?}", self.bankalar);
        println!("Kullanıcılar: {:?}", self.kullanıcılar);
        println!("Hesaplar: {:?}", self.hesaplar);
        println!("İstekler: {:?}", self.istekler);
    }
}

// Kullanıcıdan giriş alma yardımcı fonksiyonu
fn giriş_al(istem: &str) -> String {
    print!("{}", istem);
    io::stdout().flush().unwrap(); // İstemi hemen göster
    let mut giriş = String::new();
    io::stdin().read_line(&mut giriş).unwrap();
    giriş.trim().to_string()
}

fn main() {
    let mut sistem = BankacılıkSistemi::yeni();

    loop {
        println!("\nBankacılık Sistemi Menüsü:");
        println!("1. Banka oluştur");
        println!("2. Kullanıcı oluştur");
        println!("3. Hesap oluştur");
        println!("4. Para transfer et");
        println!("5. Para iste");
        println!("6. Hesaba para ekle");
        println!("7. Hesaptan para çek");
        println!("8. Geçerli durumu görüntüle");
        println!("9. Çıkış");

        let seçim = giriş_al("Seçiminizi girin (1-9): ");

        match seçim.as_str() {
            "1" => {
                let id: u32 = match giriş_al("Banka ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz ID!");
                        continue;
                    }
                };
                let isim = giriş_al("Banka ismini girin: ");
                sistem.banka_oluştur(id, isim);
                println!("Banka başarıyla oluşturuldu!");
            }
            "2" => {
                let id: u32 = match giriş_al("Kullanıcı ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz ID!");
                        continue;
                    }
                };
                let isim = giriş_al("Kullanıcı ismini girin: ");
                sistem.kullanıcı_oluştur(id, isim);
                println!("Kullanıcı başarıyla oluşturuldu!");
            }
            "3" => {
                let hesap_id: u32 = match giriş_al("Hesap ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz hesap ID'si!");
                        continue;
                    }
                };
                let banka_id: u32 = match giriş_al("Banka ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz banka ID'si!");
                        continue;
                    }
                };
                let kullanıcı_id: u32 = match giriş_al("Kullanıcı ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz kullanıcı ID'si!");
                        continue;
                    }
                };
                let başlangıç_bakiyesi: f64 = match giriş_al("Başlangıç bakiyesini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz bakiye!");
                        continue;
                    }
                };
                match sistem.hesap_oluştur(hesap_id, banka_id, kullanıcı_id, başlangıç_bakiyesi) {
                    Ok(_) => println!("Hesap başarıyla oluşturuldu!"),
                    Err(e) => println!("Hata: {}", e),
                }
            }
            "4" => {
                let gönderen_hesap_id: u32 = match giriş_al("Gönderen hesap ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz hesap ID'si!");
                        continue;
                    }
                };
                let alıcı_hesap_id: u32 = match giriş_al("Alıcı hesap ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz hesap ID'si!");
                        continue;
                    }
                };
                let miktar: f64 = match giriş_al("Transfer edilecek miktarı girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz miktar!");
                        continue;
                    }
                };
                match sistem.para_transfer_et(gönderen_hesap_id, alıcı_hesap_id, miktar) {
                    Ok(_) => println!("Para başarıyla transfer edildi!"),
                    Err(e) => println!("Hata: {}", e),
                }
            }
            "5" => {
                let gönderen_hesap_id: u32 = match giriş_al("Gönderen hesap ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz hesap ID'si!");
                        continue;
                    }
                };
                let alıcı_hesap_id: u32 = match giriş_al("Alıcı hesap ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz hesap ID'si!");
                        continue;
                    }
                };
                let miktar: f64 = match giriş_al("İstenen miktarı girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz miktar!");
                        continue;
                    }
                };
                match sistem.para_iste(gönderen_hesap_id, alıcı_hesap_id, miktar) {
                    Ok(_) => println!("Para isteği başarıyla gönderildi!"),
                    Err(e) => println!("Hata: {}", e),
                }
            }
            "6" => {
                let hesap_id: u32 = match giriş_al("Hesap ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz hesap ID'si!");
                        continue;
                    }
                };
                let miktar: f64 = match giriş_al("Eklenecek miktarı girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz miktar!");
                        continue;
                    }
                };
                match sistem.para_ekle(hesap_id, miktar) {
                    Ok(_) => println!("Para başarıyla eklendi!"),
                    Err(e) => println!("Hata: {}", e),
                }
            }
            "7" => {
                let hesap_id: u32 = match giriş_al("Hesap ID'sini girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz hesap ID'si!");
                        continue;
                    }
                };
                let miktar: f64 = match giriş_al("Çekilecek miktarı girin: ").parse() {
                    Ok(sayı) => sayı,
                    Err(_) => {
                        println!("Geçersiz miktar!");
                        continue;
                    }
                };
                match sistem.para_çek(hesap_id, miktar) {
                    Ok(_) => println!("Para başarıyla çekildi!"),
                    Err(e) => println!("Hata: {}", e),
                }
            }
            "8" => {
                sistem.durumu_görüntüle();
            }
            "9" => {
                println!("Çıkılıyor...");
                break;
            }
            _ => println!("Geçersiz seçim! Lütfen 1 ile 9 arasında bir sayı seçin."),
        }
    }
}