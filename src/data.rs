use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 65\" 4K QLED Smart TV".to_string(),
            price: 1199.99,
            description: "Stunning 4K QLED display with Quantum HDR, built-in Alexa, and a 120Hz refresh rate. Perfect for movies, gaming, and sports.".to_string(),
            image: "https://images.unsplash.com/photo-1567690187548-f07b1d7bf5a9?w=400&h=400&fit=crop".to_string(),
        },
        Product {
            id: 2,
            name: "Apple MacBook Pro 14\"".to_string(),
            price: 1999.99,
            description: "Powered by the M3 Pro chip with a stunning Liquid Retina XDR display, up to 18 hours of battery life, and a MagSafe charging port.".to_string(),
            image: "https://images.unsplash.com/photo-1517336714731-489689fd1ca8?w=400&h=400&fit=crop".to_string(),
        },
        Product {
            id: 3,
            name: "Sony WH-1000XM5 Headphones".to_string(),
            price: 349.99,
            description: "Industry-leading noise cancellation with 30-hour battery life, multipoint connection, and crystal-clear hands-free calling.".to_string(),
            image: "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=400&h=400&fit=crop".to_string(),
        },
        Product {
            id: 4,
            name: "PlayStation 5 Console".to_string(),
            price: 599.99,
            description: "Experience lightning-fast loading with the PS5's custom SSD, deeper immersion with the DualSense controller, and a new generation of gaming.".to_string(),
            image: "https://images.unsplash.com/photo-1606813907291-d86efa9b94db?w=400&h=400&fit=crop".to_string(),
        },
        Product {
            id: 5,
            name: "Apple iPhone 15 Pro".to_string(),
            price: 1199.99,
            description: "Titanium design with the A17 Pro chip, 48MP main camera system with 5x optical zoom, and Action Button for quick customization.".to_string(),
            image: "https://images.unsplash.com/photo-1510557880182-3d4d3cba35a5?w=400&h=400&fit=crop".to_string(),
        },
        Product {
            id: 6,
            name: "Samsung Galaxy Tab S9".to_string(),
            price: 799.99,
            description: "Dynamic AMOLED 2X display with S Pen included, Snapdragon 8 Gen 2 processor, and IP68 water resistance for versatile productivity.".to_string(),
            image: "https://images.unsplash.com/photo-1561154464-82e9adf32764?w=400&h=400&fit=crop".to_string(),
        },
        Product {
            id: 7,
            name: "Canon EOS R50 Camera".to_string(),
            price: 679.99,
            description: "24.2MP APS-C mirrorless camera with 4K video, Dual Pixel CMOS AF II, and a compact body ideal for creators and everyday photography.".to_string(),
            image: "https://images.unsplash.com/photo-1516035069371-29a1b244cc32?w=400&h=400&fit=crop".to_string(),
        },
        Product {
            id: 8,
            name: "Dyson V15 Detect Cordless Vacuum".to_string(),
            price: 749.99,
            description: "Laser Dust Detection reveals invisible dust on hard floors. HEPA filtration captures 99.99% of particles. Up to 60 minutes of run time.".to_string(),
            image: "https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=400&h=400&fit=crop".to_string(),
        },
        Product {
            id: 9,
            name: "Apple Watch Series 9".to_string(),
            price: 429.99,
            description: "The most powerful Apple Watch ever with the S9 chip, Double Tap gesture, always-on Retina display, and advanced health sensors.".to_string(),
            image: "https://images.unsplash.com/photo-1546868871-7041f2a55e12?w=400&h=400&fit=crop".to_string(),
        },
        Product {
            id: 10,
            name: "Bose SoundLink Max Bluetooth Speaker".to_string(),
            price: 399.99,
            description: "Premium portable Bluetooth speaker with immersive 360-degree sound, IP67 water resistance, and up to 20 hours of battery life.".to_string(),
            image: "https://images.unsplash.com/photo-1608043152269-423dbba4e7e1?w=400&h=400&fit=crop".to_string(),
        },
    ]
}