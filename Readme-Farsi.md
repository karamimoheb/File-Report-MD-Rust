
# گزارش‌گیر فایل (file-report)

ابزار خط فرمانی که با زبان راست (Rust) نوشته شده است. این برنامه یک پوشه را اسکن می‌کند، فایل‌ها را بر اساس پسوند و اندازه (اختیاری) فیلتر می‌کند و یک گزارش در قالب مارک‌دون (Markdown) تولید می‌نماید.

## ویژگی‌ها

- اسکن سریع پوشه و زیرپوشه‌ها  
- فیلتر بر اساس پسوند فایل (بدون نقطه، مثل `txt`)  
- فیلتر بر اساس حداقل یا حداکثر حجم (به مگابایت)  
- خروجی گزارش در یک فایل مارک‌دون با جزئیات شامل:  
  - مسیر نسبی هر فایل  
  - حجم فایل به مگابایت  

## نحوه ساخت (build)

برای ساخت پروژه، ابتدا اطمینان حاصل کنید که [Rust و Cargo](https://www.rust-lang.org/tools/install) روی سیستم شما نصب است. سپس دستورات زیر را اجرا کنید:

```bash
git clone https://github.com/karamimoheb/File-Report-MD-Rust
cd file-report
cargo build --release
```

فایل اجرایی در مسیر `target/release/file-report` قرار می‌گیرد.

## نحوه استفاده

```bash
file-report [OPTIONS] --path <PATH> --ext <EXT> --output <OUTPUT>
```

### پارامترهای الزامی

- `-p, --path <PATH>`: مسیر پوشه مورد نظر برای اسکن  
- `-e, --ext <EXT>`: پسوند فایل‌ها (بدون نقطه، مثلاً `txt`)  
- `-o, --output <OUTPUT>`: مسیر فایل خروجی گزارش (با فرمت مارک‌دون)

### پارامترهای اختیاری

- `--larger-than <LARGER_THAN>`: فقط فایل‌های بزرگ‌تر از این مقدار (بر حسب مگابایت) را شامل شود  
- `--smaller-than <SMALLER_THAN>`: فقط فایل‌های کوچک‌تر از این مقدار (بر حسب مگابایت) را شامل شود  
- `-h, --help`: نمایش راهنما و خروج  
- `-V, --version`: نمایش نسخه برنامه و خروج

## مثال‌ها

۱. اسکن پوشه `./documents` برای فایل‌های متنی (`txt`) و ایجاد گزارش `report.md`:

```bash
file-report --path ./documents --ext txt --output report.md
```

۲. اسکن فایل‌های `log` بزرگ‌تر از ۵ مگابایت در پوشه جاری:

```bash
file-report --path . --ext log --larger-than 5 --output large_logs.md
```

۳. فایل‌های تصویری `jpg` با حجم کمتر از ۲ مگابایت:

```bash
file-report --path ./images --ext jpg --smaller-than 2 --output small_jpgs.md
```

