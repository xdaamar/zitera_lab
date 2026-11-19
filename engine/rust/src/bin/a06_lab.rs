// A06: Insecure Design & Business Logic Flaws — Native Sandboxed Lab Binary
// Simulates NexaStore — Modern Luxury Goods & High-Performance Hardware Marketplace:
//   - Product catalog with luxury hardware items & cart drawer
//   - User wallet balance system ($50.00 starting balance)
//   - Target flag item costs $5,000.00 (Master Sovereign Escrow Keycard)
//   - Business logic flaw: Client-side price tampering & unvalidated discount override
//   - Successful procurement generates high-value executive voucher with flag
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{1n53cur3_d351gn_fl4w3d_w0rkfl0w}";
const LAB_ID: &str = "A06";
const LAB_VERSION: &str = "1.1.0";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Product {
    id: u32,
    name: String,
    category: String,
    price: f64,
    description: String,
    image_emoji: String,
    is_target_flag: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct CartItem {
    product_id: u32,
    quantity: i32,
    unit_price: f64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Order {
    order_id: String,
    product_name: String,
    total_paid: f64,
    created_at: String,
    buyer_account: String,
    voucher_secret: Option<String>,
}

struct AppState {
    products: Vec<Product>,
    cart: Vec<CartItem>,
    wallet_balance: f64,
    applied_discount: f64,
    orders: Vec<Order>,
}

impl AppState {
    fn new() -> Self {
        let products = vec![
            Product {
                id: 1,
                name: "Neural Quantum Co-Processor".to_string(),
                category: "Hardware Computing".to_string(),
                price: 1850.0,
                description: "Dedicated edge TPU matrix accelerator for low-latency neural inference.".to_string(),
                image_emoji: "🧠".to_string(),
                is_target_flag: false,
            },
            Product {
                id: 2,
                name: "Titanium Cyber Deck Workstation".to_string(),
                category: "Workstations".to_string(),
                price: 3499.0,
                description: "CNC-milled aerospace grade titanium field engineering terminal.".to_string(),
                image_emoji: "💻".to_string(),
                is_target_flag: false,
            },
            Product {
                id: 3,
                name: "Master Sovereign Escrow Keycard".to_string(),
                category: "Executive Hardware Tokens".to_string(),
                price: 5000.0,
                description: "Air-gapped physical cryptographic keycard containing Marcus Reid's SOX-404 compliance seed.".to_string(),
                image_emoji: "🗝️".to_string(),
                is_target_flag: true,
            },
            Product {
                id: 4,
                name: "Tactical YubiKey NFC Hardware Token".to_string(),
                category: "Authentication".to_string(),
                price: 50.0,
                description: "FIPS 140-3 Level 3 hardware security key with USB-C and NFC.".to_string(),
                image_emoji: "🛡️".to_string(),
                is_target_flag: false,
            },
        ];

        Self {
            products,
            cart: Vec::new(),
            wallet_balance: 50.0,
            applied_discount: 0.0,
            orders: Vec::new(),
        }
    }
}

fn css() -> &'static str {
    r#"
:root {
    --bg-page: #faf9f6;
    --bg-card: #ffffff;
    --bg-subtle: #f4f2eb;
    --border: #e7e4dc;
    --text-heading: #1c1917;
    --text-body: #44403c;
    --text-muted: #78716c;
    --primary: #d97706;
    --primary-light: #fef3c7;
    --accent-teal: #0f766e;
    --accent-rose: #e11d48;
    --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.04);
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    background: var(--bg-page);
    color: var(--text-body);
    line-height: 1.5;
    -webkit-font-smoothing: antialiased;
}
.app-container {
    max-width: 1120px;
    margin: 0 auto;
    padding: 28px 24px 60px;
}
.navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 14px 24px;
    margin-bottom: 24px;
    box-shadow: var(--shadow-sm);
}
.brand {
    display: flex;
    align-items: center;
    gap: 12px;
}
.brand-icon {
    width: 36px;
    height: 36px;
    background: linear-gradient(135deg, #d97706, #b45309);
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 18px;
    font-weight: 800;
}
.brand-title {
    font-size: 16px;
    font-weight: 800;
    color: var(--text-heading);
    letter-spacing: -0.01em;
}
.brand-sub {
    font-size: 11px;
    color: var(--text-muted);
}
.nav-links {
    display: flex;
    align-items: center;
    gap: 8px;
}
.nav-link {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted);
    text-decoration: none;
    padding: 6px 14px;
    border-radius: 6px;
    transition: all 0.15s ease;
}
.nav-link:hover {
    color: var(--text-heading);
    background: var(--bg-subtle);
}
.nav-link.active {
    color: var(--primary);
    background: var(--primary-light);
}
.wallet-badge {
    background: var(--bg-subtle);
    border: 1px solid var(--border);
    padding: 6px 14px;
    border-radius: 20px;
    font-size: 12px;
    font-weight: 700;
    color: var(--text-heading);
    display: flex;
    align-items: center;
    gap: 6px;
}
.card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 24px;
    margin-bottom: 24px;
    box-shadow: var(--shadow-sm);
}
.product-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 20px;
    margin-top: 20px;
}
.product-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    transition: transform 0.15s ease, box-shadow 0.15s ease;
}
.product-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 12px -2px rgba(0, 0, 0, 0.06);
}
.product-emoji {
    font-size: 40px;
    margin-bottom: 12px;
    display: inline-block;
}
.product-title {
    font-size: 15px;
    font-weight: 800;
    color: var(--text-heading);
    margin-bottom: 4px;
}
.product-cat {
    font-size: 11px;
    text-transform: uppercase;
    font-weight: 700;
    color: var(--text-muted);
    letter-spacing: 0.04em;
    margin-bottom: 8px;
}
.product-desc {
    font-size: 12px;
    color: var(--text-body);
    margin-bottom: 16px;
    flex-grow: 1;
}
.product-price {
    font-size: 18px;
    font-weight: 800;
    color: var(--text-heading);
}
.btn-primary {
    background: var(--primary);
    color: white;
    font-size: 13px;
    font-weight: 700;
    padding: 9px 18px;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    transition: background 0.15s ease;
}
.btn-primary:hover { background: #b45309; }
.btn-secondary {
    background: white;
    color: var(--text-heading);
    font-size: 13px;
    font-weight: 700;
    padding: 9px 18px;
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    transition: background 0.15s ease;
}
.btn-secondary:hover { background: var(--bg-subtle); }
table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    margin-top: 14px;
}
th {
    background: var(--bg-subtle);
    padding: 10px 16px;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-muted);
    text-align: left;
    border-bottom: 1px solid var(--border);
}
th:first-child { border-top-left-radius: 8px; }
th:last-child { border-top-right-radius: 8px; }
td {
    padding: 14px 16px;
    font-size: 13px;
    color: var(--text-body);
    border-bottom: 1px solid var(--border);
    vertical-align: middle;
}
tr:last-child td { border-bottom: none; }
input[type=text], input[type=number] {
    width: 100%;
    padding: 10px 14px;
    font-size: 13px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: white;
    color: var(--text-heading);
    outline: none;
}
input[type=text]:focus { border-color: var(--primary); }
.form-group { margin-bottom: 16px; }
.form-label { display: block; font-size: 12px; font-weight: 700; color: var(--text-heading); margin-bottom: 6px; }
.notice-box {
    background: var(--primary-light);
    border: 1px solid #fde68a;
    border-radius: 8px;
    padding: 14px 18px;
    font-size: 13px;
    color: #92400e;
    margin-bottom: 20px;
}
.err-box {
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 8px;
    padding: 14px 18px;
    font-size: 13px;
    color: #991b1b;
    margin-bottom: 20px;
}
.flag-banner {
    background: #ecfdf5;
    border: 1.5px solid #6ee7b7;
    border-radius: 10px;
    padding: 20px 24px;
    margin-bottom: 24px;
    display: flex;
    align-items: flex-start;
    gap: 16px;
}
.flag-val {
    font-family: monospace;
    font-size: 14px;
    font-weight: 800;
    color: #047857;
    background: #d1fae5;
    padding: 6px 14px;
    border-radius: 6px;
    margin-top: 8px;
    display: inline-block;
}
"#
}

fn render_navbar(active_tab: &str, wallet_balance: f64, cart_count: usize) -> String {
    format!(
        r#"<header class="navbar">
    <div class="brand">
        <div class="brand-icon">💎</div>
        <div>
            <div class="brand-title">NexaStore // Luxury Hardware Marketplace</div>
            <div class="brand-sub">Enterprise Procurement & Escrow Store &bull; Port 8016</div>
        </div>
    </div>
    <div style="display: flex; align-items: center; gap: 16px;">
        <nav class="nav-links">
            <a href="/" class="nav-link {cat_active}">Store Catalog</a>
            <a href="/cart" class="nav-link {cart_active}">Cart ({cart_count})</a>
            <a href="/orders" class="nav-link {ord_active}">Receipts & Orders</a>
        </nav>
        <div class="wallet-badge">
            <span>💳 Wallet:</span>
            <span style="color: #059669;">${wallet:.2}</span>
        </div>
    </div>
</header>"#,
        cat_active = if active_tab == "catalog" { "active" } else { "" },
        cart_active = if active_tab == "cart" { "active" } else { "" },
        ord_active = if active_tab == "orders" { "active" } else { "" },
        cart_count = cart_count,
        wallet = wallet_balance
    )
}

fn handle_request(
    state: &mut AppState,
    method: &str,
    path: &str,
    _headers: &HashMap<String, String>,
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    resp_headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());

    // 1. Health Probe
    if path == "/health" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "status": "ok",
            "lab": LAB_ID,
            "version": LAB_VERSION,
            "target": "NexaStore E-Commerce",
            "runtime": "native_sandboxed",
            "pid": std::process::id()
        });
        return (200, resp_headers, json.to_string());
    }

    // 2. Store Catalog (Index)
    if path == "/" || path == "/index.html" {
        let mut product_cards = String::new();
        for p in &state.products {
            let target_badge = if p.is_target_flag {
                "<div style=\"font-size: 10px; font-weight: 800; color: #b45309; background: #fef3c7; border: 1px solid #fde68a; border-radius: 4px; padding: 2px 8px; display: inline-block; margin-bottom: 8px;\">🎯 HIGH-VALUE ASSET</div>"
            } else {
                ""
            };

            product_cards.push_str(&format!(
                r#"<div class="product-card">
                    <div>
                        <div class="product-emoji">{emoji}</div>
                        {target_badge}
                        <div class="product-cat">{cat}</div>
                        <div class="product-title">{name}</div>
                        <div class="product-desc">{desc}</div>
                    </div>
                    <div>
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 14px; padding-top: 14px; border-top: 1px solid #e7e4dc;">
                            <div class="product-price">${price:.2}</div>
                            <form method="POST" action="/cart/add" style="margin: 0;">
                                <input type="hidden" name="product_id" value="{id}">
                                <input type="hidden" name="quantity" value="1">
                                <button type="submit" class="btn-primary" style="padding: 6px 14px; font-size: 12px;">Add to Cart</button>
                            </form>
                        </div>
                    </div>
                </div>"#,
                emoji = p.image_emoji,
                target_badge = target_badge,
                cat = p.category,
                name = p.name,
                desc = p.description,
                price = p.price,
                id = p.id
            ));
        }

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Store Catalog // NexaStore Luxury Hardware</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="padding: 28px;">
        <div style="display: flex; justify-content: space-between; align-items: flex-end;">
            <div>
                <h1 style="font-size: 22px; font-weight: 800; color: #1c1917;">Curated High-Value Hardware Catalog</h1>
                <p style="font-size: 13px; color: #78716c; margin-top: 4px;">
                    Enterprise procurement portal. All purchases settle against your active organization credit wallet balance.
                </p>
            </div>
            <a href="/cart" class="btn-secondary">View Shopping Cart &rarr;</a>
        </div>

        <div class="product-grid">
            {product_cards}
        </div>
    </div>

    <div class="notice-box">
        <strong>💡 Pentester Reconnaissance Note:</strong><br/>
        Notice your starting wallet balance is only <strong>$50.00</strong>, while the high-value <em>Master Sovereign Escrow Keycard</em> costs <strong>$5,000.00</strong>. Can you audit the <code>/cart/checkout</code> endpoint or HTTP POST body parameters? Does the backend trust client-submitted parameters like <code>unit_price</code>, <code>price_override</code>, or negative <code>quantity</code> values?
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("catalog", state.wallet_balance, state.cart.len()),
            product_cards = product_cards
        );
        return (200, resp_headers, html);
    }

    // 3. Add to Cart (POST)
    if path == "/cart/add" && method == "POST" {
        let mut pid: u32 = 1;
        let mut qty: i32 = 1;

        for pair in body.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                let decoded = urlencoding_decode(v);
                match k {
                    "product_id" => pid = decoded.parse::<u32>().unwrap_or(1),
                    "quantity" => qty = decoded.parse::<i32>().unwrap_or(1),
                    _ => {}
                }
            }
        }

        if let Some(p) = state.products.iter().find(|p| p.id == pid) {
            state.cart.push(CartItem {
                product_id: pid,
                quantity: qty,
                unit_price: p.price,
            });
        }

        resp_headers.insert("Location".to_string(), "/cart".to_string());
        return (302, resp_headers, "Redirecting to cart...".to_string());
    }

    // 4. Cart View & Checkout Form
    if path == "/cart" {
        let mut rows = String::new();
        let mut subtotal: f64 = 0.0;

        for item in &state.cart {
            if let Some(p) = state.products.iter().find(|p| p.id == item.product_id) {
                let line_total = item.unit_price * (item.quantity as f64);
                subtotal += line_total;

                rows.push_str(&format!(
                    r#"<tr>
                        <td>
                            <div style="font-weight: 700; color: #1c1917;">{emoji} {name}</div>
                            <div style="font-size: 11px; color: #78716c;">ID #{pid}</div>
                        </td>
                        <td>${price:.2}</td>
                        <td>{qty}</td>
                        <td style="font-weight: 700; color: #1c1917;">${line_total:.2}</td>
                    </tr>"#,
                    emoji = p.image_emoji,
                    name = p.name,
                    pid = p.id,
                    price = item.unit_price,
                    qty = item.quantity,
                    line_total = line_total
                ));
            }
        }

        let final_total = (subtotal - state.applied_discount).max(0.0);
        let has_items = !state.cart.is_empty();

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Shopping Cart // NexaStore</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="max-width: 860px; margin: 0 auto;">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
            <h2 style="font-size: 18px; font-weight: 800; color: #1c1917;">Your Procurement Cart</h2>
            <a href="/" class="btn-secondary">&larr; Continue Shopping</a>
        </div>

        <table>
            <thead>
                <tr>
                    <th>Selected Hardware Asset</th>
                    <th>Unit Price</th>
                    <th>Qty</th>
                    <th>Line Subtotal</th>
                </tr>
            </thead>
            <tbody>
                {rows}
            </tbody>
        </table>

        <div style="margin-top: 24px; padding-top: 20px; border-top: 1px solid #e7e4dc; display: flex; justify-content: space-between; align-items: flex-start;">
            <div>
                <form method="POST" action="/cart/coupon" style="display: flex; gap: 8px;">
                    <input type="text" name="coupon" placeholder="Promo code (e.g. WELCOME50)" style="max-width: 220px;">
                    <button type="submit" class="btn-secondary">Apply Promo</button>
                </form>
                <div style="font-size: 11px; color: #78716c; margin-top: 6px;">Available Promo: <code>WELCOME50</code> ($50 credit)</div>
            </div>

            <div style="text-align: right; min-width: 260px;">
                <div style="font-size: 13px; color: #78716c;">Subtotal: <strong>${subtotal:.2}</strong></div>
                <div style="font-size: 13px; color: #059669; margin: 4px 0;">Discount Applied: -${discount:.2}</div>
                <div style="font-size: 20px; font-weight: 800; color: #1c1917; margin-bottom: 16px;">Total Due: ${total:.2}</div>

                <form method="POST" action="/cart/checkout">
                    <!-- HIDDEN BUSINESS LOGIC PARAMETERS AUDITABLE BY PENTESTERS -->
                    <input type="hidden" name="cart_total" value="{total_raw}">
                    <button type="submit" class="btn-primary" style="width: 100%; justify-content: center; padding: 12px;" {btn_disabled}>
                        Complete Checkout with Wallet (${wallet:.2})
                    </button>
                </form>
            </div>
        </div>
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("cart", state.wallet_balance, state.cart.len()),
            rows = if has_items { rows } else { "<tr><td colspan='4' style='text-align:center; padding: 32px; color: #78716c;'>Your cart is currently empty. <a href='/' style='color: #d97706; font-weight: 700;'>Browse catalog &rarr;</a></td></tr>".to_string() },
            subtotal = subtotal,
            discount = state.applied_discount,
            total = final_total,
            total_raw = final_total,
            btn_disabled = if has_items { "" } else { "disabled" },
            wallet = state.wallet_balance
        );
        return (200, resp_headers, html);
    }

    // 5. Apply Promo Coupon (POST)
    if path == "/cart/coupon" && method == "POST" {
        for pair in body.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                if k == "coupon" && urlencoding_decode(v).trim().eq_ignore_ascii_case("WELCOME50") {
                    state.applied_discount += 50.0;
                }
            }
        }
        resp_headers.insert("Location".to_string(), "/cart".to_string());
        return (302, resp_headers, "Redirecting to cart...".to_string());
    }

    // 6. Checkout Processing (VULNERABLE BUSINESS LOGIC FLAW)
    if path == "/cart/checkout" && method == "POST" {
        let mut submitted_total: Option<f64> = None;
        let mut price_override: Option<f64> = None;
        let mut discount_override: Option<f64> = None;

        for pair in body.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                let decoded = urlencoding_decode(v);
                match k {
                    "cart_total" => submitted_total = decoded.parse::<f64>().ok(),
                    "price_override" | "unit_price" | "price" => price_override = decoded.parse::<f64>().ok(),
                    "discount_amount" | "discount" => discount_override = decoded.parse::<f64>().ok(),
                    _ => {}
                }
            }
        }

        // Compute actual catalog subtotal
        let actual_subtotal: f64 = state.cart.iter().map(|item| {
            if let Some(p) = state.products.iter().find(|p| p.id == item.product_id) {
                p.price * (item.quantity as f64)
            } else {
                0.0
            }
        }).sum();

        // Flaw 1: Price override parameter tampering accepted
        let mut effective_total = actual_subtotal - state.applied_discount;
        if let Some(po) = price_override {
            effective_total = po;
        } else if let Some(do_val) = discount_override {
            effective_total = (actual_subtotal - do_val).max(0.0);
        } else if let Some(st) = submitted_total {
            // Flaw 2: Trusts client-submitted cart_total hidden input!
            if st < effective_total {
                effective_total = st;
            }
        }

        if effective_total <= state.wallet_balance {
            state.wallet_balance -= effective_total;
            let order_id = format!("NEXA-ORD-{:04}", state.orders.len() + 1042);

            let has_target_keycard = state.cart.iter().any(|item| item.product_id == 3);

            let new_order = Order {
                order_id: order_id.clone(),
                product_name: if has_target_keycard {
                    "Master Sovereign Escrow Keycard (SOX-404 Compliance Seed)".to_string()
                } else {
                    "Enterprise Procurement Hardware Bundle".to_string()
                },
                total_paid: effective_total,
                created_at: "2026-11-19".to_string(),
                buyer_account: "operator.enterprise".to_string(),
                voucher_secret: if has_target_keycard { Some(FLAG.to_string()) } else { None },
            };

            state.orders.push(new_order);
            state.cart.clear();
            state.applied_discount = 0.0;

            resp_headers.insert("Location".to_string(), format!("/order/receipt?id={}", order_id));
            return (302, resp_headers, "Order placed successfully!".to_string());
        } else {
            // Insufficient funds
            let html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Payment Rejected // NexaStore</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="max-width: 680px; margin: 0 auto; text-align: center; padding: 48px 24px;">
        <div style="font-size: 38px; margin-bottom: 12px;">💳❌</div>
        <h2 style="font-size: 18px; font-weight: 800; color: #1c1917; margin-bottom: 8px;">Insufficient Wallet Funds</h2>
        <p style="font-size: 13px; color: #78716c; margin-bottom: 20px;">
            The order total (<strong>${total:.2}</strong>) exceeds your current available enterprise wallet credit (<strong>${wallet:.2}</strong>).
        </p>

        <div class="err-box" style="text-align: left;">
            <strong>Procurement Violation:</strong> Transaction declined. Additional credit must be allocated or payment terms restructured.
        </div>

        <div style="display: flex; justify-content: center; gap: 12px;">
            <a href="/cart" class="btn-primary">&larr; Return to Cart</a>
            <a href="/" class="btn-secondary">Browse Other Items</a>
        </div>
    </div>
</div>
</body>
</html>"#,
                css = css(),
                navbar = render_navbar("cart", state.wallet_balance, state.cart.len()),
                total = effective_total,
                wallet = state.wallet_balance
            );
            return (402, resp_headers, html);
        }
    }

    // 7. Order Receipt (Reveals Flag on Item #3)
    if path.starts_with("/order/receipt") || path.starts_with("/orders") {
        let order_id = path.split('?').nth(1).and_then(|q| {
            for pair in q.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    if k == "id" {
                        return Some(urlencoding_decode(v));
                    }
                }
            }
            None
        });

        let target_order = if let Some(ref oid) = order_id {
            state.orders.iter().find(|o| o.order_id == *oid)
        } else {
            state.orders.last()
        };

        if let Some(o) = target_order {
            let flag_box = if let Some(ref secret) = o.voucher_secret {
                format!(
                    r#"<div class="flag-banner">
                        <div style="font-size: 32px;">🏆</div>
                        <div>
                            <div style="font-size: 16px; font-weight: 800; color: #065f46;">BUSINESS LOGIC FLAW EXPLOITED!</div>
                            <p style="font-size: 13px; color: #047857; margin-top: 4px;">
                                You successfully bypassed procurement cost validation and purchased the $5,000.00 Sovereign Escrow Keycard for only ${paid:.2}!
                            </p>
                            <div style="font-size: 12px; color: #047857; margin-top: 8px;">
                                Executive Voucher Hardware Seed (Flag):
                            </div>
                            <div class="flag-val">{secret}</div>
                        </div>
                    </div>"#,
                    paid = o.total_paid,
                    secret = secret
                )
            } else {
                "".to_string()
            };

            let html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Order Receipt // NexaStore</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="max-width: 760px; margin: 0 auto;">
        {flag_box}

        <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 20px;">
            <div>
                <span style="font-size: 11px; font-weight: 700; color: #059669; background: #ecfdf5; border: 1px solid #a7f3d0; padding: 2px 8px; border-radius: 4px;">PAID &amp; SETTLED</span>
                <h1 style="font-size: 20px; font-weight: 800; color: #1c1917; margin-top: 6px;">Invoice Receipt &bull; {oid}</h1>
            </div>
            <a href="/" class="btn-secondary">&larr; Return to Store</a>
        </div>

        <table>
            <tbody>
                <tr>
                    <td><strong>Purchased Hardware Item:</strong></td>
                    <td>{pname}</td>
                </tr>
                <tr>
                    <td><strong>Settlement Amount:</strong></td>
                    <td style="font-weight: 800; color: #1c1917;">${paid:.2}</td>
                </tr>
                <tr>
                    <td><strong>Buyer Account:</strong></td>
                    <td><code>{buyer}</code></td>
                </tr>
                <tr>
                    <td><strong>Timestamp:</strong></td>
                    <td>{date}</td>
                </tr>
            </tbody>
        </table>
    </div>
</div>
</body>
</html>"#,
                css = css(),
                navbar = render_navbar("orders", state.wallet_balance, state.cart.len()),
                flag_box = flag_box,
                oid = o.order_id,
                pname = o.product_name,
                paid = o.total_paid,
                buyer = o.buyer_account,
                date = o.created_at
            );
            return (200, resp_headers, html);
        } else {
            let html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Orders // NexaStore</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="text-align: center; padding: 48px 24px;">
        <h2 style="font-size: 18px; font-weight: 800; color: #1c1917; margin-bottom: 8px;">No Completed Orders Found</h2>
        <p style="font-size: 13px; color: #78716c; margin-bottom: 20px;">
            You have not completed any hardware procurement checkouts yet.
        </p>
        <a href="/" class="btn-primary">Browse Store Catalog &rarr;</a>
    </div>
</div>
</body>
</html>"#,
                css = css(),
                navbar = render_navbar("orders", state.wallet_balance, state.cart.len())
            );
            return (200, resp_headers, html);
        }
    }

    // Default 404
    (
        404,
        resp_headers,
        format!(
            "<!DOCTYPE html><html><head><style>{}</style></head><body><div class='app-container'><div class='card' style='text-align:center;'><h2>404 — Endpoint Not Found</h2><p style='margin-top:10px;'><a href='/' class='btn-secondary'>Return to Store</a></p></div></div></body></html>",
            css()
        ),
    )
}

fn urlencoding_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(c1), Some(c2)) = (h1, h2) {
                let hex_str = format!("{}{}", c1, c2);
                if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                    result.push(byte as char);
                    continue;
                }
            }
        } else if ch == '+' {
            result.push(' ');
        } else {
            result.push(ch);
        }
    }
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("NexaStore Luxury Hardware (Insecure Design & Business Logic Flaw) v{}", LAB_VERSION);
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a06-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A06 v{} (native_sandboxed // NexaStore Marketplace)", LAB_VERSION);
        return;
    }

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut state = AppState::new();

    // 1. Emit READY handshake
    let ready = serde_json::json!({
        "type": "READY",
        "lab_id": LAB_ID,
        "version": LAB_VERSION
    });
    let _ = writeln!(stdout, "{}", ready);
    let _ = stdout.flush();

    // 2. Stdio message loop
    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let val: serde_json::Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let msg_type = val.get("type").and_then(|v| v.as_str()).unwrap_or("");
        match msg_type {
            "HTTP_REQUEST" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let method = val.get("method").and_then(|v| v.as_str()).unwrap_or("GET");
                let path = val.get("path").and_then(|v| v.as_str()).unwrap_or("/");
                let body = val.get("body").and_then(|v| v.as_str()).unwrap_or("");

                let mut headers = HashMap::new();
                if let Some(h_obj) = val.get("headers").and_then(|v| v.as_object()) {
                    for (k, v) in h_obj {
                        if let Some(s) = v.as_str() {
                            headers.insert(k.to_lowercase(), s.to_string());
                        }
                    }
                }

                let (status, resp_headers, resp_body) =
                    handle_request(&mut state, method, path, &headers, body);

                let resp_json = serde_json::json!({
                    "type": "HTTP_RESPONSE",
                    "id": id,
                    "status": status,
                    "headers": resp_headers,
                    "body": resp_body
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "HEALTH" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let resp_json = serde_json::json!({
                    "type": "HEALTH_OK",
                    "id": id,
                    "lab_id": LAB_ID,
                    "status": "HEALTHY"
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "RESET" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                state = AppState::new();
                let resp_json = serde_json::json!({
                    "type": "RESET_OK",
                    "id": id,
                    "lab_id": LAB_ID
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "SHUTDOWN" => {
                exit(0);
            }
            _ => {}
        }
    }
}
