import os
from PIL import Image, ImageDraw, ImageEnhance, ImageFilter
import numpy as np

AVATARS_DIR = r"ui/flutter/assets/images/avatars"

def load_rgba(name):
    path = os.path.join(AVATARS_DIR, name)
    return Image.open(path).convert("RGBA")

def save_avatar(img, name):
    path = os.path.join(AVATARS_DIR, name)
    # Ensure exact 256x256 RGBA
    if img.size != (256, 256):
        img = img.resize((256, 256), Image.Resampling.LANCZOS)
    img.save(path)
    print(f"Saved: {path} ({img.size[0]}x{img.size[1]})")

def get_hsv_arrays(img):
    arr = np.array(img).astype(np.float32)
    r, g, b = arr[:, :, 0] / 255.0, arr[:, :, 1] / 255.0, arr[:, :, 2] / 255.0
    cmax = np.maximum(np.maximum(r, g), b)
    cmin = np.minimum(np.minimum(r, g), b)
    delta = cmax - cmin

    h = np.zeros_like(r)
    mask_r = (cmax == r) & (delta != 0)
    mask_g = (cmax == g) & (delta != 0)
    mask_b = (cmax == b) & (delta != 0)

    h[mask_r] = 60.0 * (((g[mask_r] - b[mask_r]) / delta[mask_r]) % 6.0)
    h[mask_g] = 60.0 * (((b[mask_g] - r[mask_g]) / delta[mask_g]) + 2.0)
    h[mask_b] = 60.0 * (((r[mask_b] - g[mask_b]) / delta[mask_b]) + 4.0)

    s = np.zeros_like(r)
    s[cmax != 0] = delta[cmax != 0] / cmax[cmax != 0]
    v = cmax
    return arr, h, s, v

def hsv_to_rgba(arr_orig, h, s, v):
    c = v * s
    x = c * (1.0 - np.abs((h / 60.0) % 2.0 - 1.0))
    m_val = v - c

    new_r = np.zeros_like(h)
    new_g = np.zeros_like(h)
    new_b = np.zeros_like(h)

    conds = [
        ((h >= 0) & (h < 60), (c, x, 0)),
        ((h >= 60) & (h < 120), (x, c, 0)),
        ((h >= 120) & (h < 180), (0, c, x)),
        ((h >= 180) & (h < 240), (0, x, c)),
        ((h >= 240) & (h < 300), (x, 0, c)),
        ((h >= 300) & (h < 360), (c, 0, x)),
    ]

    for cond, (cr, cg, cb) in conds:
        cb_arr = np.full_like(h, cb) if isinstance(cb, (int, float)) else cb
        cg_arr = np.full_like(h, cg) if isinstance(cg, (int, float)) else cg
        cr_arr = np.full_like(h, cr) if isinstance(cr, (int, float)) else cr
        new_r[cond] = cr_arr[cond] + m_val[cond]
        new_g[cond] = cg_arr[cond] + m_val[cond]
        new_b[cond] = cb_arr[cond] + m_val[cond]

    rgb = np.stack([new_r, new_g, new_b], axis=-1) * 255.0
    res = np.clip(rgb, 0, 255).astype(np.uint8)
    alpha = arr_orig[:, :, 3].astype(np.uint8)
    return Image.fromarray(np.dstack([res, alpha]), "RGBA")

def is_hair_pixel(arr, y_max=180):
    r, g, b = arr[:, :, 0], arr[:, :, 1], arr[:, :, 2]
    # Background: pastel pink
    is_bg = (r > 215) & (g > 185) & (b > 185) & (np.abs(r.astype(int) - b.astype(int)) < 55)
    # Skin: high R, medium G, lower B
    is_skin = (r > 200) & (g > 140) & (b > 115) & (r > g) & (g > b)
    # Upper head region
    y_idx, x_idx = np.indices((256, 256))
    in_box = (y_idx < y_max) & (x_idx > 15) & (x_idx < 241) & (y_idx > 15)
    return in_box & ~is_bg & ~is_skin

# ==========================================
# 1. GIRL_SERIOUS: Hana (Smart Analyst with Glasses)
# ==========================================
def create_girl_serious():
    rin = load_rgba("girl_cool.png")
    arr, h, s, v = get_hsv_arrays(rin)
    hair = is_hair_pixel(arr, y_max=175)

    # Shift hair to deep emerald / dark teal-black
    h[hair] = 165.0
    s[hair] = np.clip(s[hair] * 0.7, 0.2, 0.6)
    v[hair] = np.clip(v[hair] * 0.75, 0.15, 0.7)

    res = hsv_to_rgba(arr, h, s, v)

    # Draw chic red-rimmed anime glasses and straight analytical mouth
    draw = ImageDraw.Draw(res)
    # Glasses: left frame around (85..115, 100..120), right around (140..170, 100..120)
    glass_color = (220, 38, 38, 240) # sleek crimson frame
    glass_rim = (153, 27, 27, 255)
    glint = (255, 255, 255, 180)

    # Left frame
    draw.rounded_rectangle([82, 100, 116, 120], radius=4, outline=glass_rim, width=2)
    draw.line([85, 102, 95, 102], fill=glint, width=1)
    # Right frame
    draw.rounded_rectangle([138, 100, 172, 120], radius=4, outline=glass_rim, width=2)
    draw.line([141, 102, 151, 102], fill=glint, width=1)
    # Bridge across nose
    draw.line([116, 108, 138, 108], fill=glass_rim, width=2)
    # Temples to ears
    draw.line([76, 106, 82, 106], fill=glass_rim, width=2)
    draw.line([172, 106, 178, 106], fill=glass_rim, width=2)

    # Focused analytical mouth line
    draw.line([120, 142, 136, 142], fill=(185, 75, 60, 240), width=2)
    save_avatar(res, "girl_serious.png")

# ==========================================
# 2. GIRL_POUTY: Yuki (Tsundere Pout, Lilac/Silver Hair)
# ==========================================
def create_girl_pouty():
    aoi = load_rgba("girl_cheerful.png")
    arr, h, s, v = get_hsv_arrays(aoi)
    hair = is_hair_pixel(arr, y_max=170)

    # Shift hair to platinum silver-lavender
    h[hair] = 275.0
    s[hair] = np.clip(s[hair] * 0.25, 0.05, 0.25)
    v[hair] = np.clip(v[hair] * 1.25, 0.6, 1.0)

    res = hsv_to_rgba(arr, h, s, v)
    draw = ImageDraw.Draw(res)

    # Puffy cheeks blush: soft pink circular blushes
    blush_color = (255, 120, 145, 120)
    draw.ellipse([76, 122, 100, 138], fill=blush_color)
    draw.ellipse([156, 122, 180, 138], fill=blush_color)

    # Cute pout mouth (inverted small arc / cute >3< puff)
    # Cover old mouth
    draw.rectangle([118, 134, 138, 146], fill=(253, 210, 195, 255))
    # Draw pout
    pout_color = (210, 70, 80, 250)
    draw.arc([121, 137, 135, 147], start=200, end=340, fill=pout_color, width=2)
    draw.point([128, 141], fill=(255, 180, 190, 255))

    save_avatar(res, "girl_pouty.png")

# ==========================================
# 3. GIRL_FIERCE: Asuka (Auburn Hair, Headband, Confident)
# ==========================================
def create_girl_fierce():
    zeta = load_rgba("zeta_default.png")
    arr, h, s, v = get_hsv_arrays(zeta)
    hair = is_hair_pixel(arr, y_max=185)

    # Shift hair to vibrant fiery auburn/crimson
    h[hair] = 12.0
    s[hair] = np.clip(s[hair] * 1.4, 0.45, 0.85)
    v[hair] = np.clip(v[hair] * 1.1, 0.3, 0.85)

    res = hsv_to_rgba(arr, h, s, v)
    draw = ImageDraw.Draw(res)

    # Red cyber headband across forehead (around y: 64..74, x: 65..190)
    hb_color = (225, 29, 72, 250)
    hb_border = (159, 18, 57, 255)
    draw.rectangle([68, 66, 188, 76], fill=hb_color, outline=hb_border, width=1)
    # Cyber metal core badge in center
    draw.rectangle([120, 64, 136, 78], fill=(241, 245, 249, 255), outline=(15, 23, 42, 255), width=1)
    draw.ellipse([124, 67, 132, 75], fill=(6, 182, 212, 255))

    # Fierce determined eyebrows
    brow_color = (136, 19, 55, 240)
    draw.line([84, 91, 110, 96], fill=brow_color, width=3)
    draw.line([146, 96, 172, 91], fill=brow_color, width=3)

    # Confident sharp smirk
    draw.line([120, 142, 136, 138], fill=(190, 24, 93, 255), width=2)
    save_avatar(res, "girl_fierce.png")

# ==========================================
# 4. BOY_CHEERFUL: Ren (Messy Blonde Boy, Warm Smile)
# ==========================================
def create_boy_cheerful():
    aoi = load_rgba("girl_cheerful.png")
    arr, h, s, v = get_hsv_arrays(aoi)
    hair = is_hair_pixel(arr, y_max=175)

    # Shift hair to bright anime honey-blonde
    h[hair] = 42.0
    s[hair] = np.clip(s[hair] * 1.3, 0.5, 0.9)
    v[hair] = np.clip(v[hair] * 1.25, 0.6, 1.0)

    res = hsv_to_rgba(arr, h, s, v)
    draw = ImageDraw.Draw(res)

    # Mask long side curls to turn into boy haircut
    # Replace outer hair pixels (x < 65 or x > 190, y: 100..180) with background
    bg_color = (250, 230, 232, 255)
    border_color = (244, 196, 202, 255)

    draw.rectangle([25, 105, 68, 175], fill=bg_color)
    draw.rectangle([188, 105, 231, 175], fill=bg_color)

    # Redraw boy sideburns and messy boy hair tufts
    hair_base = (234, 179, 8, 255)
    hair_shadow = (180, 83, 9, 255)
    hair_high = (254, 240, 138, 255)

    # Left boy sideburn
    draw.polygon([(68, 105), (78, 135), (66, 130)], fill=hair_shadow)
    draw.polygon([(69, 105), (76, 132), (70, 128)], fill=hair_base)
    # Right boy sideburn
    draw.polygon([(188, 105), (178, 135), (190, 130)], fill=hair_shadow)
    draw.polygon([(187, 105), (180, 132), (186, 128)], fill=hair_base)

    # Spiky top tufts
    draw.polygon([(118, 48), (128, 32), (135, 48)], fill=hair_base)
    draw.polygon([(122, 48), (128, 35), (130, 48)], fill=hair_high)

    # Confident boy grin
    draw.line([120, 140, 136, 140], fill=(180, 60, 45, 255), width=2)
    draw.line([122, 141, 134, 145], fill=(220, 80, 65, 255), width=2)

    save_avatar(res, "boy_cheerful.png")

# ==========================================
# 5. BOY_COOL: Kaito (Teal/Dark Undercut, Cyber Goggles)
# ==========================================
def create_boy_cool():
    rin = load_rgba("girl_cool.png")
    arr, h, s, v = get_hsv_arrays(rin)
    hair = is_hair_pixel(arr, y_max=175)

    # Shift hair to stylish cool teal/dark charcoal
    h[hair] = 182.0
    s[hair] = np.clip(s[hair] * 0.9, 0.3, 0.7)
    v[hair] = np.clip(v[hair] * 0.7, 0.2, 0.6)

    res = hsv_to_rgba(arr, h, s, v)
    draw = ImageDraw.Draw(res)

    bg_color = (250, 230, 232, 255)
    # Trim sides to boy undercut
    draw.rectangle([25, 110, 68, 180], fill=bg_color)
    draw.rectangle([188, 110, 231, 180], fill=bg_color)

    # Add Cyber Goggles pushed up on hair (y: 52..68, x: 80..176)
    goggle_strap = (30, 41, 59, 255)
    goggle_frame = (15, 118, 110, 255)
    goggle_lens = (34, 211, 238, 240)
    goggle_glint = (255, 255, 255, 200)

    # Strap
    draw.line([60, 60, 196, 60], fill=goggle_strap, width=5)
    # Left lens frame
    draw.rounded_rectangle([86, 50, 122, 70], radius=5, fill=goggle_frame, outline=(13, 148, 136, 255), width=2)
    draw.rounded_rectangle([90, 54, 118, 66], radius=3, fill=goggle_lens)
    draw.line([93, 56, 103, 56], fill=goggle_glint, width=2)
    # Right lens frame
    draw.rounded_rectangle([134, 50, 170, 70], radius=5, fill=goggle_frame, outline=(13, 148, 136, 255), width=2)
    draw.rounded_rectangle([138, 54, 166, 66], radius=3, fill=goggle_lens)
    draw.line([141, 56, 151, 56], fill=goggle_glint, width=2)
    # Bridge
    draw.line([122, 60, 134, 60], fill=goggle_frame, width=3)

    # Cool subtle smirk
    draw.line([122, 140, 138, 137], fill=(160, 50, 50, 255), width=2)

    save_avatar(res, "boy_cool.png")

# ==========================================
# 6. BOY_SMUG: Shin (Silver Spiky Hair, Winking, Playful)
# ==========================================
def create_boy_smug():
    mia = load_rgba("girl_smug.png")
    arr, h, s, v = get_hsv_arrays(mia)
    hair = is_hair_pixel(arr, y_max=175)

    # Shift hair to pure silver-white with subtle ice-blue tone
    h[hair] = 205.0
    s[hair] = np.clip(s[hair] * 0.18, 0.05, 0.2)
    v[hair] = np.clip(v[hair] * 1.35, 0.7, 1.0)

    res = hsv_to_rgba(arr, h, s, v)
    draw = ImageDraw.Draw(res)

    bg_color = (250, 230, 232, 255)
    # Trim twin-tails to messy anime boy haircut
    draw.rectangle([25, 75, 70, 185], fill=bg_color)
    draw.rectangle([186, 75, 231, 185], fill=bg_color)

    # Draw spiky messy silver boy hair spikes
    silver_base = (241, 245, 249, 255)
    silver_shadow = (148, 163, 184, 255)
    silver_highlight = (255, 255, 255, 255)

    # Side boy spikes
    draw.polygon([(70, 100), (58, 120), (74, 125)], fill=silver_shadow)
    draw.polygon([(71, 102), (62, 118), (73, 122)], fill=silver_base)
    draw.polygon([(186, 100), (198, 120), (182, 125)], fill=silver_shadow)
    draw.polygon([(185, 102), (194, 118), (183, 122)], fill=silver_base)

    # Top messy boy spikes
    draw.polygon([(105, 45), (115, 26), (125, 45)], fill=silver_base)
    draw.polygon([(130, 45), (142, 28), (150, 48)], fill=silver_base)
    draw.polygon([(132, 45), (140, 32), (146, 46)], fill=silver_highlight)

    # Accentuate winking anime smirk
    draw.line([122, 137, 138, 134], fill=(160, 40, 50, 255), width=2)
    save_avatar(res, "boy_smug.png")

# ==========================================
# 7. BOY_SERIOUS: Daiki (Navy Boy, Cyber Scouter / Visor)
# ==========================================
def create_boy_serious():
    rin = load_rgba("girl_cool.png")
    arr, h, s, v = get_hsv_arrays(rin)
    hair = is_hair_pixel(arr, y_max=175)

    # Shift hair to deep navy blue
    h[hair] = 225.0
    s[hair] = np.clip(s[hair] * 1.1, 0.4, 0.8)
    v[hair] = np.clip(v[hair] * 0.65, 0.15, 0.5)

    res = hsv_to_rgba(arr, h, s, v)
    draw = ImageDraw.Draw(res)

    bg_color = (250, 230, 232, 255)
    draw.rectangle([25, 105, 68, 180], fill=bg_color)
    draw.rectangle([188, 105, 231, 180], fill=bg_color)

    # Tactical Cyber Scouter over left eye (x: 82..120, y: 96..122)
    scouter_frame = (30, 41, 59, 255)
    scouter_glass = (16, 185, 129, 210) # Glowing tactical emerald
    scouter_hud = (52, 211, 153, 255)

    # Ear mount
    draw.rectangle([68, 102, 82, 118], fill=scouter_frame)
    # Lens
    draw.polygon([(82, 100), (124, 102), (118, 122), (84, 120)], fill=scouter_glass, outline=scouter_frame)
    # HUD reticle lines
    draw.line([95, 111, 107, 111], fill=scouter_hud, width=1)
    draw.line([101, 105, 101, 117], fill=scouter_hud, width=1)
    draw.rectangle([98, 108, 104, 114], outline=scouter_hud, width=1)

    # Straight serious line mouth
    draw.line([120, 142, 136, 142], fill=(160, 50, 50, 255), width=2)
    save_avatar(res, "boy_serious.png")

# ==========================================
# 8. BOY_POUTY: Sora (Soft Caramel Hair, Tsundere Blush)
# ==========================================
def create_boy_pouty():
    aoi = load_rgba("girl_cheerful.png")
    arr, h, s, v = get_hsv_arrays(aoi)
    hair = is_hair_pixel(arr, y_max=175)

    # Shift hair to soft warm milk-tea / caramel brown
    h[hair] = 28.0
    s[hair] = np.clip(s[hair] * 0.7, 0.3, 0.6)
    v[hair] = np.clip(v[hair] * 1.05, 0.45, 0.8)

    res = hsv_to_rgba(arr, h, s, v)
    draw = ImageDraw.Draw(res)

    bg_color = (250, 230, 232, 255)
    draw.rectangle([25, 105, 68, 175], fill=bg_color)
    draw.rectangle([188, 105, 231, 175], fill=bg_color)

    # Soft boy side tufts
    tuft_color = (180, 115, 75, 255)
    draw.polygon([(68, 105), (74, 128), (68, 124)], fill=tuft_color)
    draw.polygon([(188, 105), (182, 128), (188, 124)], fill=tuft_color)

    # Tsundere flustered boy blush
    blush_color = (255, 110, 130, 130)
    draw.ellipse([76, 122, 100, 136], fill=blush_color)
    draw.ellipse([156, 122, 180, 136], fill=blush_color)

    # Soft pout mouth
    draw.rectangle([118, 135, 138, 145], fill=(253, 210, 195, 255))
    draw.arc([121, 137, 135, 146], start=210, end=330, fill=(200, 70, 70, 255), width=2)

    save_avatar(res, "boy_pouty.png")

# ==========================================
# 9. BOY_FIERCE: Ryu (Spiky Red Boy, Gaming Headset)
# ==========================================
def create_boy_fierce():
    zeta = load_rgba("zeta_default.png")
    arr, h, s, v = get_hsv_arrays(zeta)
    hair = is_hair_pixel(arr, y_max=185)

    # Shift hair to fiery anime red
    h[hair] = 4.0
    s[hair] = np.clip(s[hair] * 1.5, 0.6, 0.95)
    v[hair] = np.clip(v[hair] * 1.2, 0.4, 0.9)

    res = hsv_to_rgba(arr, h, s, v)
    draw = ImageDraw.Draw(res)

    bg_color = (250, 230, 232, 255)
    # Trim long twin-tails
    draw.rectangle([25, 70, 68, 190], fill=bg_color)
    draw.rectangle([188, 70, 231, 190], fill=bg_color)

    # Spiky boy hair outline & shapes
    red_base = (220, 38, 38, 255)
    red_dark = (153, 27, 27, 255)
    red_light = (248, 113, 113, 255)

    # Spiky side tufts
    draw.polygon([(68, 90), (52, 112), (70, 120)], fill=red_dark)
    draw.polygon([(69, 93), (56, 110), (70, 116)], fill=red_base)
    draw.polygon([(188, 90), (204, 112), (186, 120)], fill=red_dark)
    draw.polygon([(187, 93), (200, 110), (186, 116)], fill=red_base)

    # Spiky top hair locks
    draw.polygon([(112, 45), (124, 22), (134, 45)], fill=red_base)
    draw.polygon([(116, 45), (124, 26), (130, 45)], fill=red_light)
    draw.polygon([(132, 45), (146, 25), (154, 46)], fill=red_base)

    # Gaming headset on ear (neon orange accents)
    draw.rectangle([182, 98, 196, 126], fill=(15, 23, 42, 255), outline=(249, 115, 22, 255), width=2)
    draw.ellipse([185, 106, 193, 118], fill=(249, 115, 22, 255))

    # Competitive fierce grin
    draw.polygon([(120, 140), (136, 137), (132, 145)], fill=(220, 38, 38, 255))
    draw.line([120, 140, 136, 137], fill=(153, 27, 27, 255), width=2)

    save_avatar(res, "boy_fierce.png")

# Run all
if __name__ == "__main__":
    print("Generating authentic anime pixel art avatars matching Zeta...")
    create_girl_serious()
    create_girl_pouty()
    create_girl_fierce()
    create_boy_cheerful()
    create_boy_cool()
    create_boy_smug()
    create_boy_serious()
    create_boy_pouty()
    create_boy_fierce()
    print("All 9 avatars synthesized successfully in authentic Zeta pixel art style!")
