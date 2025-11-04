# Changelog Kit

Semua perubahan penting pada Kit tercatat di sini. Format ini mengikuti Keep a Changelog dan Semantic Versioning.

---

## [1.0.0] - 2025-01-04

### 🚀 Initial Stable Release

Rilis stabil pertama Kit, utilitas CLI serbaguna yang cepat dan dapat disesuaikan.
**Author:** danko1122q

### ✨ Added (Fitur Baru)

* **Syntax Highlighting:** Dukungan untuk lebih dari 200 bahasa pemrograman dan markup menggunakan syntect.
* **File Viewing & Paging:**

  * Tampilan berkas dengan penomoran baris dan integrasi Git di sidebar.
  * Paging otomatis untuk berkas besar dengan dukungan pager kustom.
* **File Management:**

  * Membuat berkas dengan flag `-c` / `--create` (mirip `touch`).
  * Membuat direktori rekursif dengan flag `--mkdir` (mirip `mkdir -p`).
* **Git Integration:** Menampilkan modifikasi dan perubahan berkas di sidebar.
* **Customization & Configuration:**

  * Dukungan penuh untuk berkas konfigurasi.
  * Custom syntax mapping untuk pola berkas tertentu.
  * Tema warna siap pakai untuk berbagai latar belakang terminal.

### ⚙️ Configuration & Environment

* Direktori standar:

  * Konfigurasi: `~/.config/kit`
  * Cache: `~/.cache/kit`
* Variabel lingkungan:

  * `KIT_THEME`, `KIT_STYLE`, `KIT_PAGER`, `KIT_PAGING`, `KIT_TABS`
  * `KIT_CONFIG_PATH`, `KIT_CACHE_PATH` untuk lokasi berkas kustom

### 📦 Technical Details

* Language: Rust (Edition 2021)
* Minimum Rust Version: 1.70
* License: MIT OR Apache-2.0
* Copyright: © 2025 danko1122q. Semua hak dilindungi undang-undang.
