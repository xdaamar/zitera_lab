import os
import math
from PIL import Image, ImageDraw, ImageFont, ImageFilter

OUTPUT_DIR = "ui/flutter/assets/images/avatars"
os.makedirs(OUTPUT_DIR, exist_ok=True)

AVATAR_SPECS = [
    {
        "id": "girl_serious",
        "name": "Hana",
        "gender": "female",
        "bg_top": (26, 16, 60),
        "bg_bot": (10, 8, 30),
        "accent": (168, 85, 247), # Purple
        "hair": (76, 29, 149),
        "skin": (255, 236, 224),
        "eye": (192, 132, 252),
        "hair_type": "bob_glasses",
        "expression": "serious",
        "accessory": "glasses",
    },
    {
        "id": "girl_pouty",
        "name": "Yuki",
        "gender": "female",
        "bg_top": (40, 15, 30),
        "bg_bot": (15, 6, 15),
        "accent": (244, 114, 182), # Pink
        "hair": (251, 146, 60),
        "skin": (255, 240, 235),
        "eye": (236, 72, 153),
        "hair_type": "side_pony",
        "expression": "pouty",
        "accessory": "cyber_hairpin",
    },
    {
        "id": "girl_fierce",
        "name": "Asuka",
        "gender": "female",
        "bg_top": (50, 15, 15),
        "bg_bot": (20, 5, 5),
        "accent": (239, 68, 68), # Crimson
        "hair": (185, 28, 28),
        "skin": (255, 235, 220),
        "eye": (248, 113, 113),
        "hair_type": "wild_spiky",
        "expression": "fierce",
        "accessory": "headband",
    },
    {
        "id": "boy_cheerful",
        "name": "Ren",
        "gender": "male",
        "bg_top": (15, 35, 50),
        "bg_bot": (6, 15, 25),
        "accent": (56, 189, 248), # Sky blue
        "hair": (245, 158, 11),  # Golden blonde
        "skin": (255, 238, 225),
        "eye": (14, 165, 233),
        "hair_type": "spiky_messy",
        "expression": "cheerful",
        "accessory": "hoodie",
    },
    {
        "id": "boy_cool",
        "name": "Kaito",
        "gender": "male",
        "bg_top": (10, 25, 40),
        "bg_bot": (4, 10, 20),
        "accent": (20, 184, 166), # Teal
        "hair": (15, 118, 110),
        "skin": (250, 235, 222),
        "eye": (45, 212, 191),
        "hair_type": "undercut",
        "expression": "cool",
        "accessory": "cyber_goggles",
    },
    {
        "id": "boy_smug",
        "name": "Shin",
        "gender": "male",
        "bg_top": (30, 20, 45),
        "bg_bot": (12, 8, 20),
        "accent": (168, 85, 247), # Indigo
        "hair": (203, 213, 225), # Silver
        "skin": (255, 240, 230),
        "eye": (147, 51, 234),
        "hair_type": "swept_bangs",
        "expression": "smug",
        "accessory": "earring",
    },
    {
        "id": "boy_serious",
        "name": "Daiki",
        "gender": "male",
        "bg_top": (15, 20, 35),
        "bg_bot": (5, 8, 15),
        "accent": (96, 165, 250), # Blue
        "hair": (30, 41, 59),   # Navy
        "skin": (250, 234, 220),
        "eye": (59, 130, 246),
        "hair_type": "slicked",
        "expression": "serious",
        "accessory": "visor",
    },
    {
        "id": "boy_pouty",
        "name": "Sora",
        "gender": "male",
        "bg_top": (35, 25, 20),
        "bg_bot": (15, 10, 8),
        "accent": (251, 146, 60), # Amber
        "hair": (120, 53, 15),  # Brown
        "skin": (255, 238, 225),
        "eye": (217, 119, 6),
        "hair_type": "soft_bangs",
        "expression": "pouty",
        "accessory": "choker",
    },
    {
        "id": "boy_fierce",
        "name": "Ryu",
        "gender": "male",
        "bg_top": (45, 15, 20),
        "bg_bot": (18, 5, 8),
        "accent": (239, 68, 68), # Red
        "hair": (153, 27, 27),
        "skin": (255, 235, 220),
        "eye": (220, 38, 38),
        "hair_type": "spiky_up",
        "expression": "fierce",
        "accessory": "gaming_headset",
    },
]

def draw_anime_avatar(spec):
    size = 512
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(im)

    # 1. Cyberpunk radial background
    cx, cy = size // 2, size // 2
    for r in range(size // 2, 0, -2):
        t = r / (size // 2)
        r_col = int(spec["bg_top"][0] * (1 - t) + spec["bg_bot"][0] * t)
        g_col = int(spec["bg_top"][1] * (1 - t) + spec["bg_bot"][1] * t)
        b_col = int(spec["bg_top"][2] * (1 - t) + spec["bg_bot"][2] * t)
        draw.ellipse([cx - r, cy - r, cx + r, cy + r], fill=(r_col, g_col, b_col, 255))

    # Tech HUD accent rings
    acc = spec["accent"]
    draw.arc([16, 16, size - 16, size - 16], start=30, end=150, fill=acc + (180,), width=3)
    draw.arc([16, 16, size - 16, size - 16], start=210, end=330, fill=acc + (180,), width=3)
    draw.arc([28, 28, size - 28, size - 28], start=0, end=360, fill=acc + (60,), width=1)

    # 2. Techwear Jacket / Shoulders
    jacket_col = (15, 23, 42, 255) # Slate 900
    trim_col = acc + (255,)
    draw.polygon([
        (80, 512), (140, 390), (256, 420), (372, 390), (432, 512)
    ], fill=jacket_col)
    # Jacket collar & zipper
    draw.line([(256, 420), (256, 512)], fill=trim_col, width=4)
    draw.polygon([(180, 400), (256, 440), (230, 480)], fill=(30, 41, 59, 255))
    draw.polygon([(332, 400), (256, 440), (282, 480)], fill=(30, 41, 59, 255))
    draw.line([(140, 390), (256, 440)], fill=trim_col, width=3)
    draw.line([(372, 390), (256, 440)], fill=trim_col, width=3)

    # 3. Neck
    skin = spec["skin"] + (255,)
    draw.rectangle([226, 320, 286, 410], fill=(skin[0] - 15, skin[1] - 15, skin[2] - 15, 255))
    # Neck shadow
    draw.polygon([(226, 320), (286, 320), (256, 360)], fill=(skin[0] - 30, skin[1] - 30, skin[2] - 30, 255))

    # 4. Head / Face shape
    draw.polygon([
        (160, 210), (160, 280), (210, 350), (256, 380), (302, 350), (352, 280), (352, 210), (256, 170)
    ], fill=skin)

    # Cheeks blush
    blush_col = (244, 114, 182, 140) if spec["gender"] == "female" or spec["expression"] == "pouty" else (248, 113, 113, 90)
    draw.ellipse([180, 290, 225, 310], fill=blush_col)
    draw.ellipse([287, 290, 332, 310], fill=blush_col)

    # 5. Eyes
    eye_col = spec["eye"] + (255,)
    pupil_col = (20, 20, 30, 255)
    
    # Left eye
    draw.ellipse([190, 245, 235, 295], fill=(255, 255, 255, 255))
    draw.ellipse([198, 250, 230, 292], fill=eye_col)
    draw.ellipse([204, 258, 224, 284], fill=pupil_col)
    draw.ellipse([202, 254, 212, 264], fill=(255, 255, 255, 255)) # Catchlight
    draw.ellipse([216, 276, 222, 282], fill=(255, 255, 255, 200)) # Small reflection

    # Right eye
    draw.ellipse([277, 245, 322, 295], fill=(255, 255, 255, 255))
    draw.ellipse([282, 250, 314, 292], fill=eye_col)
    draw.ellipse([288, 258, 308, 284], fill=pupil_col)
    draw.ellipse([286, 254, 296, 264], fill=(255, 255, 255, 255))
    draw.ellipse([300, 276, 306, 282], fill=(255, 255, 255, 200))

    # Eyelashes / Eyebrows by expression
    eyelash_col = (30, 20, 40, 255)
    expr = spec["expression"]
    if expr == "cheerful":
        # Happy arched brows & curved lash
        draw.arc([186, 235, 238, 260], start=190, end=350, fill=eyelash_col, width=4)
        draw.arc([273, 235, 325, 260], start=190, end=350, fill=eyelash_col, width=4)
        draw.arc([186, 220, 238, 240], start=190, end=350, fill=eyelash_col, width=3)
        draw.arc([273, 220, 325, 240], start=190, end=350, fill=eyelash_col, width=3)
    elif expr == "smug":
        # Smug raised brow, right eye winking
        draw.arc([186, 235, 238, 260], start=190, end=350, fill=eyelash_col, width=4)
        draw.line([(188, 220), (236, 225)], fill=eyelash_col, width=3)
        # Wink on right eye
        draw.rectangle([277, 245, 322, 295], fill=skin)
        draw.arc([275, 255, 325, 285], start=20, end=160, fill=eyelash_col, width=4)
        draw.line([(275, 215), (323, 225)], fill=eyelash_col, width=3)
    elif expr == "serious":
        # Angled firm brows
        draw.line([(188, 246), (238, 242)], fill=eyelash_col, width=4)
        draw.line([(274, 242), (324, 246)], fill=eyelash_col, width=4)
        draw.line([(190, 230), (238, 238)], fill=eyelash_col, width=3)
        draw.line([(274, 238), (322, 230)], fill=eyelash_col, width=3)
    elif expr == "pouty":
        # Troubled / pouty angled brows
        draw.line([(188, 244), (238, 246)], fill=eyelash_col, width=4)
        draw.line([(274, 246), (324, 244)], fill=eyelash_col, width=4)
        draw.line([(190, 235), (238, 226)], fill=eyelash_col, width=3)
        draw.line([(274, 226), (322, 235)], fill=eyelash_col, width=3)
    elif expr == "fierce":
        # Sharp slanted angry/fierce brows
        draw.line([(188, 248), (238, 240)], fill=eyelash_col, width=4)
        draw.line([(274, 240), (324, 248)], fill=eyelash_col, width=4)
        draw.line([(190, 238), (238, 225)], fill=eyelash_col, width=4)
        draw.line([(274, 225), (322, 238)], fill=eyelash_col, width=4)
    else: # cool
        draw.line([(188, 244), (238, 242)], fill=eyelash_col, width=4)
        draw.line([(274, 242), (324, 244)], fill=eyelash_col, width=4)
        draw.line([(190, 228), (238, 230)], fill=eyelash_col, width=3)
        draw.line([(274, 230), (322, 228)], fill=eyelash_col, width=3)

    # Nose
    draw.line([(254, 292), (258, 302)], fill=(eyelash_col[0], eyelash_col[1], eyelash_col[2], 120), width=2)

    # Mouth by expression
    if expr == "cheerful":
        # Open radiant anime smile
        draw.polygon([(242, 330), (270, 330), (256, 348)], fill=(225, 29, 72, 255))
        draw.arc([242, 320, 270, 348], start=0, end=180, fill=eyelash_col, width=3)
    elif expr == "smug":
        # Cat :3 smirk
        draw.arc([240, 325, 256, 340], start=0, end=180, fill=eyelash_col, width=3)
        draw.arc([256, 325, 272, 340], start=0, end=180, fill=eyelash_col, width=3)
    elif expr == "pouty":
        # Puffed pout curve
        draw.arc([246, 330, 266, 344], start=180, end=360, fill=(225, 29, 72, 255), width=4)
    elif expr == "fierce":
        # Fierce grin
        draw.line([(244, 338), (268, 332)], fill=eyelash_col, width=3)
        draw.line([(246, 338), (266, 344)], fill=eyelash_col, width=2)
    elif expr == "serious":
        # Neutral firm line
        draw.line([(246, 335), (266, 335)], fill=eyelash_col, width=3)
    else: # cool
        # Slight confident smirk
        draw.line([(248, 336), (268, 333)], fill=eyelash_col, width=3)

    # 6. Hair
    hair_col = spec["hair"] + (255,)
    hair_type = spec["hair_type"]

    if "bob" in hair_type or "side_pony" in hair_type or spec["gender"] == "female":
        # Female anime hair
        draw.polygon([
            (150, 190), (140, 340), (165, 360), (180, 280), (200, 220), (256, 150)
        ], fill=hair_col)
        draw.polygon([
            (362, 190), (372, 340), (347, 360), (332, 280), (312, 220), (256, 150)
        ], fill=hair_col)
        # Bangs
        draw.polygon([(170, 160), (180, 235), (205, 170), (220, 240), (256, 180), (292, 240), (307, 170), (332, 235), (342, 160), (256, 130)], fill=hair_col)
        # Hair highlight sheen
        draw.line([(190, 185), (322, 185)], fill=acc + (180,), width=3)
    else:
        # Male spiky / anime undercut hair
        draw.polygon([
            (145, 210), (135, 160), (160, 180), (165, 130), (195, 155), (215, 110),
            (256, 140), (297, 110), (317, 155), (347, 130), (352, 180), (377, 160), (367, 210), (256, 160)
        ], fill=hair_col)
        # Spiky bangs
        draw.polygon([(165, 180), (185, 235), (210, 190), (235, 245), (256, 195), (277, 245), (302, 190), (327, 235), (347, 180)], fill=hair_col)
        draw.line([(180, 170), (332, 170)], fill=acc + (180,), width=3)

    # 7. Accessories
    acc_type = spec["accessory"]
    if acc_type == "glasses":
        draw.rectangle([186, 248, 236, 288], outline=acc + (255,), width=3)
        draw.rectangle([276, 248, 326, 288], outline=acc + (255,), width=3)
        draw.line([(236, 268), (276, 268)], fill=acc + (255,), width=3)
        # Glare
        draw.line([(190, 252), (210, 284)], fill=(255, 255, 255, 140), width=2)
        draw.line([(280, 252), (300, 284)], fill=(255, 255, 255, 140), width=2)
    elif acc_type == "cyber_goggles" or acc_type == "visor":
        draw.rectangle([170, 190, 342, 225], fill=(15, 23, 42, 220), outline=acc + (255,), width=3)
        draw.line([(180, 207), (332, 207)], fill=acc + (255,), width=4)
    elif acc_type == "gaming_headset":
        # Ear cups
        draw.rectangle([130, 230, 160, 290], fill=(20, 20, 25, 255), outline=acc + (255,), width=3)
        draw.rectangle([352, 230, 382, 290], fill=(20, 20, 25, 255), outline=acc + (255,), width=3)
        draw.arc([130, 120, 382, 300], start=180, end=360, fill=(30, 30, 40, 255), width=8)

    # Clean border ring
    draw.ellipse([4, 4, size - 4, size - 4], outline=acc + (255,), width=4)

    out_path = os.path.join(OUTPUT_DIR, f"{spec['id']}.png")
    im.save(out_path, "PNG")
    print(f"Created {spec['id']}.png -> {out_path}")

for spec in AVATAR_SPECS:
    draw_anime_avatar(spec)
print("All avatar portraits generated successfully!")
