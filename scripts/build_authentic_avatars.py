"""
Zitera Lab Operative Avatar Generator
Generates clean, authentic 16-bit retro pixel art anime avatars matching Zeta's exact aesthetic:
- 6 Authentic Anime Boys (Ren, Kaito, Shin, Daiki, Sora, Ryu)
- 3 Polished Anime Girls (Hana, Yuki, Asuka)
Zero glitched boxes, zero rectangles, clean pixel art framing and palette.
"""

import os
import math
import numpy as np
from PIL import Image, ImageDraw, ImageFilter

OUTPUT_DIR = "ui/flutter/assets/images/avatars"
os.makedirs(OUTPUT_DIR, exist_ok=True)

def create_card_background(size, bg_color, border_color, outer_color=None):
    """Creates an authentic retro pixel-art framed card background matching Zeta."""
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(im)
    
    if outer_color is None:
        # Subtle complementary outer border tint
        outer_color = tuple(max(0, c - 25) for c in bg_color[:3]) + (255,)
        
    # Outer rounded card
    draw.rounded_rectangle([2, 2, size - 3, size - 3], radius=16, fill=outer_color)
    # Inner border line
    draw.rounded_rectangle([5, 5, size - 6, size - 6], radius=13, fill=bg_color, outline=border_color, width=2)
    
    # Corner pixel art notches / accents
    notch_col = border_color
    draw.line([(8, 16), (16, 8)], fill=notch_col, width=1)
    draw.line([(size - 9, 16), (size - 17, 8)], fill=notch_col, width=1)
    draw.line([(8, size - 17), (16, size - 9)], fill=notch_col, width=1)
    draw.line([(size - 9, size - 17), (size - 17, size - 9)], fill=notch_col, width=1)
    
    return im

def build_character_avatar(spec):
    size = 256
    bg_img = create_card_background(size, spec["bg_color"], spec["border_color"], spec.get("outer_color"))
    
    # Create character layer
    char_img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(char_img)
    
    is_male = spec["gender"] == "male"
    
    # ── 1. BODY & CYBER JACKET ─────────────────────────────────────────────
    # Shoulders and torso
    jacket_col = spec.get("jacket_color", (245, 248, 252, 255))
    accent_col = spec["accent_color"]
    dark_trim = (30, 41, 59, 255)
    shadow_col = (203, 213, 225, 255)
    
    if is_male:
        # Broad masculine shoulders
        # Base torso
        draw.polygon([
            (32, 256), (42, 205), (78, 175), (105, 170), 
            (151, 170), (178, 175), (214, 205), (224, 256)
        ], fill=jacket_col)
        
        # Outer jacket shading / contours
        draw.polygon([(32, 256), (42, 205), (60, 210), (50, 256)], fill=shadow_col)
        draw.polygon([(224, 256), (214, 205), (196, 210), (206, 256)], fill=shadow_col)
        
        # Cyber collar (high tech collar)
        draw.polygon([(92, 185), (82, 155), (108, 162), (128, 172), (148, 162), (174, 155), (164, 185), (128, 192)], fill=jacket_col)
        draw.line([(82, 155), (92, 185)], fill=dark_trim, width=2)
        draw.line([(174, 155), (164, 185)], fill=dark_trim, width=2)
        draw.line([(92, 185), (128, 192), (164, 185)], fill=dark_trim, width=2)
        
        # Inner shirt / undershirt
        draw.polygon([(108, 162), (128, 172), (148, 162), (142, 185), (114, 185)], fill=(20, 24, 33, 255))
        
        # Accent lines / chest seams
        draw.line([(88, 180), (70, 256)], fill=accent_col, width=3)
        draw.line([(168, 180), (186, 256)], fill=accent_col, width=3)
        draw.line([(128, 172), (128, 256)], fill=dark_trim, width=2) # Central zipper
        
        # Cyber Lab chest badge
        draw.rectangle([145, 195, 172, 222], fill=(255, 255, 255, 255), outline=accent_col, width=1)
        draw.line([(152, 208), (165, 208)], fill=accent_col, width=2)
        draw.line([(158, 202), (158, 215)], fill=accent_col, width=2)
        
        # Shoulder tech patch
        draw.rectangle([48, 220, 64, 242], fill=accent_col)
        draw.rectangle([52, 224, 60, 238], fill=(255, 255, 255, 255))
    else:
        # Female anime jacket (Zeta-style fitted cyber coat)
        draw.polygon([
            (40, 256), (50, 212), (82, 178), (105, 172), 
            (151, 172), (174, 178), (206, 212), (216, 256)
        ], fill=jacket_col)
        draw.polygon([(40, 256), (50, 212), (65, 215), (55, 256)], fill=shadow_col)
        draw.polygon([(216, 256), (206, 212), (191, 215), (201, 256)], fill=shadow_col)
        
        # Fitted high collar
        draw.polygon([(94, 182), (86, 156), (110, 164), (128, 172), (146, 164), (170, 156), (162, 182), (128, 188)], fill=jacket_col)
        draw.line([(86, 156), (94, 182)], fill=dark_trim, width=2)
        draw.line([(170, 156), (162, 182)], fill=dark_trim, width=2)
        draw.line([(94, 182), (128, 188), (162, 182)], fill=dark_trim, width=2)
        
        # Black undershirt
        draw.polygon([(110, 164), (128, 172), (146, 164), (140, 184), (116, 184)], fill=(18, 18, 24, 255))
        
        # Accent seams
        draw.line([(92, 182), (78, 256)], fill=accent_col, width=3)
        draw.line([(164, 182), (178, 256)], fill=accent_col, width=3)
        draw.line([(128, 172), (128, 256)], fill=dark_trim, width=2)
        
        # Emblem
        draw.rectangle([142, 192, 168, 218], fill=(255, 255, 255, 255), outline=accent_col, width=1)
        draw.ellipse([148, 198, 162, 212], outline=accent_col, width=1)
        draw.line([(155, 198), (155, 212)], fill=accent_col, width=1)
        draw.line([(148, 205), (162, 205)], fill=accent_col, width=1)

    # ── 2. NECK & HEAD BASE ───────────────────────────────────────────────
    skin_base = spec.get("skin_color", (255, 240, 235, 255))
    skin_shadow = spec.get("skin_shadow", (245, 205, 195, 255))
    skin_outline = (160, 95, 85, 255)
    
    # Neck
    draw.polygon([(114, 150), (142, 150), (144, 175), (112, 175)], fill=skin_shadow)
    draw.polygon([(116, 150), (140, 150), (138, 172), (118, 172)], fill=skin_base)
    
    # Jaw & Face
    if is_male:
        # Masculine jaw (slightly sharper angles and flatter chin)
        jaw_pts = [
            (82, 95), (80, 120), (88, 142), (105, 162), 
            (122, 170), (134, 170), (151, 162), (168, 142), 
            (176, 120), (174, 95)
        ]
    else:
        # Feminine jaw (soft rounded curve)
        jaw_pts = [
            (84, 98), (82, 122), (90, 142), (108, 160), 
            (128, 168), (148, 160), (166, 142), (174, 122), 
            (172, 98)
        ]
        
    draw.polygon(jaw_pts, fill=skin_base)
    # Jaw outline
    for i in range(len(jaw_pts) - 1):
        draw.line([jaw_pts[i], jaw_pts[i+1]], fill=skin_outline, width=1)
        
    # Chin shadow
    if is_male:
        draw.polygon([(118, 166), (138, 166), (134, 170), (122, 170)], fill=skin_shadow)
    else:
        draw.polygon([(122, 164), (134, 164), (128, 168)], fill=skin_shadow)

    # Ears
    draw.polygon([(78, 115), (74, 128), (82, 138), (84, 125)], fill=skin_base)
    draw.line([(78, 115), (74, 128), (82, 138)], fill=skin_outline, width=1)
    draw.polygon([(178, 115), (182, 128), (174, 138), (172, 125)], fill=skin_base)
    draw.line([(178, 115), (182, 128), (174, 138)], fill=skin_outline, width=1)

    # ── 3. EYES & EXPRESSION ──────────────────────────────────────────────
    eye_col = spec["eye_color"]
    eye_dark = tuple(max(0, c - 70) for c in eye_col[:3]) + (255,)
    eye_bright = tuple(min(255, c + 50) for c in eye_col[:3]) + (255,)
    pupil_col = (15, 15, 25, 255)
    
    expr = spec.get("expression", "cheerful")
    
    # Left eye center: (102, 118), Right eye center: (154, 118)
    lx, rx, ey = 102, 154, 118
    
    # Blush
    blush_col = (255, 170, 180, 140) if not is_male else (255, 190, 185, 90)
    if expr in ["cheerful", "pouty"]:
        draw.ellipse([lx - 12, ey + 10, lx + 12, ey + 20], fill=blush_col)
        draw.ellipse([rx - 12, ey + 10, rx + 12, ey + 20], fill=blush_col)
    
    # Eyeball whites
    if is_male:
        # Male eye shape: cooler, sleeker horizontal almond shape
        draw.ellipse([lx - 14, ey - 7, lx + 14, ey + 8], fill=(255, 255, 255, 255))
        draw.ellipse([rx - 14, ey - 7, rx + 14, ey + 8], fill=(255, 255, 255, 255))
        
        # Iris
        if expr == "smug":
            # Winking right eye or half-lidded
            # Left iris
            draw.ellipse([lx - 9, ey - 6, lx + 7, ey + 8], fill=eye_col)
            draw.ellipse([lx - 7, ey - 4, lx + 5, ey + 6], fill=eye_dark)
            draw.ellipse([lx - 4, ey - 2, lx + 2, ey + 4], fill=pupil_col)
            draw.rectangle([lx - 6, ey - 5, lx - 3, ey - 2], fill=(255, 255, 255, 255)) # Glint
            
            # Right eye: confident winking curve
            draw.arc([rx - 14, ey - 4, rx + 14, ey + 10], start=190, end=350, fill=pupil_col, width=3)
            draw.line([(rx - 14, ey + 4), (rx + 14, ey + 4)], fill=pupil_col, width=2)
        else:
            # Both eyes open (cool / serious / cheerful / fierce)
            for cx in [lx, rx]:
                draw.ellipse([cx - 8, ey - 6, cx + 8, ey + 8], fill=eye_col)
                draw.ellipse([cx - 6, ey - 4, cx + 6, ey + 6], fill=eye_dark)
                draw.ellipse([cx - 3, ey - 2, cx + 3, ey + 4], fill=pupil_col)
                # Iris bottom gradient
                draw.arc([cx - 7, ey + 1, cx + 7, ey + 7], start=0, end=180, fill=eye_bright, width=2)
                # White highlight glint
                draw.rectangle([cx - 5, ey - 4, cx - 2, ey - 1], fill=(255, 255, 255, 255))
                draw.rectangle([cx + 2, ey + 2, cx + 4, ey + 4], fill=(255, 255, 255, 220))
                
        # Strong masculine upper eyelid lash
        if expr != "smug":
            draw.line([(lx - 16, ey - 8), (lx + 15, ey - 7)], fill=pupil_col, width=3)
            draw.line([(rx - 15, ey - 7), (rx + 16, ey - 8)], fill=pupil_col, width=3)
        else:
            draw.line([(lx - 16, ey - 8), (lx + 15, ey - 7)], fill=pupil_col, width=3)
            
        draw.line([(lx - 10, ey + 8), (lx + 8, ey + 8)], fill=skin_outline, width=1)
        if expr != "smug":
            draw.line([(rx - 8, ey + 8), (rx + 10, ey + 8)], fill=skin_outline, width=1)

    else:
        # Female anime eyes (Zeta large sparkling anime eyes)
        draw.ellipse([lx - 16, ey - 10, lx + 16, ey + 11], fill=(255, 255, 255, 255))
        draw.ellipse([rx - 16, ey - 10, rx + 16, ey + 11], fill=(255, 255, 255, 255))
        
        for cx in [lx, rx]:
            draw.ellipse([cx - 10, ey - 9, cx + 10, ey + 11], fill=eye_col)
            draw.ellipse([cx - 8, ey - 6, cx + 8, ey + 8], fill=eye_dark)
            draw.ellipse([cx - 4, ey - 3, cx + 4, ey + 5], fill=pupil_col)
            draw.arc([cx - 9, ey + 2, cx + 9, ey + 10], start=0, end=180, fill=eye_bright, width=3)
            # Large twin highlights
            draw.ellipse([cx - 7, ey - 6, cx - 2, ey - 1], fill=(255, 255, 255, 255))
            draw.ellipse([cx + 2, ey + 2, cx + 5, ey + 5], fill=(255, 255, 255, 240))
            
        # Feminine curved dark eyelash line
        draw.arc([lx - 18, ey - 14, lx + 18, ey + 2], start=190, end=350, fill=pupil_col, width=4)
        draw.arc([rx - 18, ey - 14, rx + 18, ey + 2], start=190, end=350, fill=pupil_col, width=4)
        # Side flutter lashes
        draw.line([(lx - 17, ey - 8), (lx - 20, ey - 12)], fill=pupil_col, width=2)
        draw.line([(rx + 17, ey - 8), (rx + 20, ey - 12)], fill=pupil_col, width=2)
        draw.line([(lx - 10, ey + 11), (lx + 10, ey + 11)], fill=skin_outline, width=1)
        draw.line([(rx - 10, ey + 11), (rx + 10, ey + 11)], fill=skin_outline, width=1)

    # Eyebrows
    brow_col = spec.get("hair_color", pupil_col)
    brow_dark = tuple(max(0, c - 40) for c in brow_col[:3]) + (255,)
    
    if is_male:
        if expr == "cheerful":
            # Energetic high arched brows
            draw.line([(lx - 14, ey - 16), (lx + 12, ey - 19)], fill=brow_dark, width=3)
            draw.line([(rx - 12, ey - 19), (rx + 14, ey - 16)], fill=brow_dark, width=3)
        elif expr == "cool":
            # Calm straight confident brows
            draw.line([(lx - 14, ey - 16), (lx + 14, ey - 16)], fill=brow_dark, width=3)
            draw.line([(rx - 14, ey - 16), (rx + 14, ey - 16)], fill=brow_dark, width=3)
        elif expr == "smug":
            # One cocky raised brow, one lowered
            draw.line([(lx - 14, ey - 22), (lx + 12, ey - 18)], fill=brow_dark, width=3)
            draw.line([(rx - 12, ey - 14), (rx + 14, ey - 16)], fill=brow_dark, width=3)
        elif expr == "serious":
            # Sharp focused angled brows
            draw.line([(lx - 14, ey - 15), (lx + 12, ey - 19)], fill=brow_dark, width=3)
            draw.line([(rx - 12, ey - 19), (rx + 14, ey - 15)], fill=brow_dark, width=3)
        elif expr == "pouty":
            # Pouty troubled brows
            draw.line([(lx - 14, ey - 18), (lx + 12, ey - 15)], fill=brow_dark, width=3)
            draw.line([(rx - 12, ey - 15), (rx + 14, ey - 18)], fill=brow_dark, width=3)
        elif expr == "fierce":
            # Angry / fierce slanted brows
            draw.line([(lx - 14, ey - 14), (lx + 14, ey - 20)], fill=brow_dark, width=3)
            draw.line([(rx - 14, ey - 20), (rx + 14, ey - 14)], fill=brow_dark, width=3)
    else:
        # Female fine arched eyebrows
        if expr == "serious":
            draw.line([(lx - 14, ey - 18), (lx + 12, ey - 20)], fill=brow_dark, width=2)
            draw.line([(rx - 12, ey - 20), (rx + 14, ey - 18)], fill=brow_dark, width=2)
        elif expr == "pouty":
            draw.line([(lx - 14, ey - 20), (lx + 12, ey - 17)], fill=brow_dark, width=2)
            draw.line([(rx - 12, ey - 17), (rx + 14, ey - 20)], fill=brow_dark, width=2)
        elif expr == "fierce":
            draw.line([(lx - 14, ey - 16), (lx + 14, ey - 22)], fill=brow_dark, width=2)
            draw.line([(rx - 14, ey - 22), (rx + 14, ey - 16)], fill=brow_dark, width=2)

    # Nose
    draw.rectangle([127, 128, 129, 131], fill=skin_outline)
    draw.rectangle([126, 130, 128, 132], fill=(220, 140, 130, 255))

    # Mouth
    lip_col = (190, 60, 70, 255)
    lip_dark = (120, 30, 40, 255)
    
    if expr == "cheerful":
        # Wide happy open anime smile with teeth
        draw.polygon([(118, 140), (138, 140), (128, 150)], fill=lip_col)
        draw.polygon([(121, 140), (135, 140), (128, 144)], fill=(255, 255, 255, 255))
        draw.line([(117, 139), (139, 139)], fill=lip_dark, width=2)
    elif expr == "smug":
        # Cat :3 smirk or cocky sideways smile
        draw.arc([118, 137, 128, 146], start=0, end=180, fill=lip_dark, width=2)
        draw.arc([128, 137, 138, 146], start=0, end=180, fill=lip_dark, width=2)
        draw.line([(137, 142), (142, 139)], fill=lip_dark, width=2)
    elif expr == "pouty":
        # Cute inverted pout curve (cemberut imut)
        draw.arc([122, 140, 134, 148], start=180, end=360, fill=lip_dark, width=3)
        draw.ellipse([124, 144, 132, 148], fill=lip_col)
    elif expr == "fierce":
        # Confident sharp smirk
        draw.line([(120, 144), (136, 140)], fill=lip_dark, width=2)
        draw.line([(126, 144), (136, 140)], fill=lip_col, width=2)
        draw.polygon([(128, 142), (134, 140), (132, 145)], fill=(255, 255, 255, 255)) # Fang glint
    elif expr == "serious":
        # Clean neutral firm line
        draw.line([(122, 142), (134, 142)], fill=lip_dark, width=2)
    else: # cool
        # Subtle slight smile
        draw.line([(122, 142), (135, 141)], fill=lip_dark, width=2)

    # ── 4. HAIRSTYLE (AUTHENTIC ANIME LAYERS) ─────────────────────────────
    h_col = spec["hair_color"]
    h_shadow = tuple(max(0, c - 50) for c in h_col[:3]) + (255,)
    h_high = tuple(min(255, c + 45) for c in h_col[:3]) + (255,)
    h_line = tuple(max(0, c - 85) for c in h_col[:3]) + (255,)
    
    hair_style = spec.get("hair_style", "boy_spiky")
    
    if hair_style == "boy_spiky":
        # Ren (Messy energetic spiky boy hair)
        # Crown volume and spikes
        spikes = [
            (82, 95), (65, 80), (74, 62), (60, 48), (82, 42), (90, 24), (112, 34),
            (128, 18), (144, 34), (166, 24), (174, 42), (196, 48), (182, 62), 
            (191, 80), (174, 95), (172, 125), (162, 115), (145, 102), (128, 105),
            (111, 102), (94, 115), (84, 125)
        ]
        draw.polygon(spikes, fill=h_col)
        # Bangs falling over forehead
        draw.polygon([(88, 92), (98, 110), (106, 92), (118, 114), (128, 96), (138, 114), (148, 92), (158, 110), (168, 92)], fill=h_col)
        # Hair sheen band
        draw.line([(88, 55), (168, 55)], fill=h_high, width=4)
        # Layering shadows
        draw.polygon([(65, 80), (74, 62), (80, 72)], fill=h_shadow)
        draw.polygon([(191, 80), (182, 62), (176, 72)], fill=h_shadow)
        
    elif hair_style == "boy_cool_bangs":
        # Kaito (Cool layered hair with side-swept bangs)
        hair_pts = [
            (82, 95), (70, 75), (72, 50), (88, 32), (112, 24), (135, 22), (158, 26),
            (178, 38), (186, 58), (180, 85), (172, 120), (164, 108), (150, 96),
            (130, 98), (115, 96), (98, 108), (84, 120)
        ]
        draw.polygon(hair_pts, fill=h_col)
        # Cool long diagonal bangs covering one side
        draw.polygon([(82, 85), (92, 118), (105, 92), (115, 122), (128, 95), (142, 116), (154, 90), (165, 105), (174, 85)], fill=h_col)
        draw.line([(85, 48), (170, 48)], fill=h_high, width=3)
        draw.polygon([(70, 75), (78, 95), (82, 80)], fill=h_shadow)
        
    elif hair_style == "boy_messy_silver":
        # Shin (Messy silver hair with ahoge / cowlick)
        hair_pts = [
            (82, 95), (66, 76), (72, 54), (82, 40), (100, 26), (122, 20), (134, 20),
            (156, 26), (174, 40), (184, 54), (190, 76), (174, 95), (168, 122),
            (158, 110), (144, 98), (128, 102), (112, 98), (98, 110), (88, 122)
        ]
        draw.polygon(hair_pts, fill=h_col)
        # Ahoge (playful cowlick on top)
        draw.arc([118, 6, 138, 28], start=180, end=350, fill=h_col, width=4)
        draw.line([(128, 6), (134, 14)], fill=h_high, width=2)
        # Spiky jagged bangs
        draw.polygon([(86, 90), (96, 112), (108, 92), (120, 116), (130, 96), (140, 118), (152, 92), (164, 112), (172, 90)], fill=h_col)
        draw.line([(84, 45), (172, 45)], fill=h_high, width=4)
        
    elif hair_style == "boy_slicked_visor":
        # Daiki (Neat styled dark hair)
        hair_pts = [
            (82, 95), (74, 75), (78, 50), (94, 30), (118, 22), (138, 22), (162, 30),
            (178, 50), (182, 75), (174, 95), (168, 118), (158, 104), (142, 94),
            (128, 96), (114, 94), (98, 104), (88, 118)
        ]
        draw.polygon(hair_pts, fill=h_col)
        # Neat side-parted bangs
        draw.polygon([(84, 88), (98, 106), (110, 90), (124, 108), (138, 92), (152, 106), (166, 88)], fill=h_col)
        draw.line([(90, 42), (166, 42)], fill=h_high, width=3)
        
    elif hair_style == "boy_fluffy_pout":
        # Sora (Fluffy rounded blonde hair)
        hair_pts = [
            (82, 95), (68, 80), (70, 56), (82, 38), (102, 24), (128, 20), (154, 24),
            (174, 38), (186, 56), (188, 80), (174, 95), (170, 122), (158, 110),
            (145, 98), (128, 100), (111, 98), (98, 110), (86, 122)
        ]
        draw.polygon(hair_pts, fill=h_col)
        # Fluffy wavy bangs
        draw.polygon([(86, 90), (96, 112), (108, 94), (122, 114), (134, 94), (148, 114), (160, 90), (170, 108)], fill=h_col)
        draw.line([(86, 46), (170, 46)], fill=h_high, width=4)
        
    elif hair_style == "boy_fierce_up":
        # Ryu (Spiky dynamic crimson hair pointing up)
        spikes = [
            (82, 95), (66, 75), (62, 50), (76, 38), (86, 18), (108, 26), (120, 10),
            (136, 10), (148, 26), (170, 18), (180, 38), (194, 50), (190, 75),
            (174, 95), (170, 124), (158, 112), (145, 100), (128, 102), (111, 100),
            (98, 112), (86, 124)
        ]
        draw.polygon(spikes, fill=h_col)
        # Sharp angular bangs
        draw.polygon([(86, 90), (94, 114), (106, 92), (116, 118), (128, 96), (140, 118), (150, 92), (162, 114), (170, 90)], fill=h_col)
        draw.line([(88, 48), (168, 48)], fill=h_high, width=3)
        
    elif hair_style == "girl_sleek_dark":
        # Hana (Sleek straight dark violet hair - cleanly behind shoulders, NO blocks!)
        # Back hair silhouette
        draw.polygon([
            (68, 85), (62, 140), (66, 190), (78, 220), (92, 185),
            (164, 185), (178, 220), (190, 190), (194, 140), (188, 85)
        ], fill=h_shadow)
        # Main head hair
        hair_pts = [
            (80, 95), (70, 70), (75, 45), (92, 28), (118, 20), (138, 20), (164, 28),
            (181, 45), (186, 70), (176, 95), (172, 130), (164, 115), (152, 98),
            (128, 100), (104, 98), (92, 115), (84, 130)
        ]
        draw.polygon(hair_pts, fill=h_col)
        # Neat straight hime-cut bangs
        draw.polygon([(86, 90), (90, 110), (106, 110), (108, 95), (126, 112), (130, 112), (148, 95), (150, 110), (166, 110), (170, 90)], fill=h_col)
        draw.line([(88, 44), (168, 44)], fill=h_high, width=3)
        
    elif hair_style == "girl_wavy_pink":
        # Yuki (Cute soft wavy peach/pink bob with cute clips)
        hair_pts = [
            (78, 95), (66, 80), (68, 52), (84, 32), (106, 20), (128, 18), (150, 20),
            (172, 32), (188, 52), (190, 80), (178, 95), (178, 135), (168, 120),
            (152, 102), (128, 104), (104, 102), (88, 120), (78, 135)
        ]
        draw.polygon(hair_pts, fill=h_col)
        # Side bob curl
        draw.polygon([(68, 115), (62, 142), (76, 150), (84, 130)], fill=h_col)
        draw.polygon([(188, 115), (194, 142), (180, 150), (172, 130)], fill=h_col)
        # Cute rounded bangs
        draw.polygon([(84, 90), (94, 112), (106, 94), (120, 116), (136, 116), (150, 94), (162, 112), (172, 90)], fill=h_col)
        draw.line([(86, 44), (170, 44)], fill=h_high, width=4)
        
    elif hair_style == "girl_fierce_tails":
        # Asuka (Fiery red hair with cute high side ponytails cleanly integrated)
        # High ponytail bases (NO rectangular boxes, natural clean hair silhouette)
        draw.polygon([(52, 45), (40, 75), (46, 115), (62, 125), (74, 90), (66, 55)], fill=h_col)
        draw.polygon([(204, 45), (216, 75), (210, 115), (194, 125), (182, 90), (190, 55)], fill=h_col)
        # Hair ties
        draw.rectangle([56, 52, 68, 62], fill=accent_col)
        draw.rectangle([188, 52, 200, 62], fill=accent_col)
        # Crown
        hair_pts = [
            (80, 95), (72, 72), (78, 48), (96, 30), (118, 22), (138, 22), (160, 30),
            (178, 48), (184, 72), (176, 95), (170, 120), (158, 105), (142, 96),
            (128, 98), (114, 96), (98, 105), (86, 120)
        ]
        draw.polygon(hair_pts, fill=h_col)
        # Bangs
        draw.polygon([(86, 90), (96, 115), (108, 92), (120, 118), (136, 118), (148, 92), (160, 115), (170, 90)], fill=h_col)
        draw.line([(88, 44), (168, 44)], fill=h_high, width=3)

    # ── 5. ACCESSORIES ────────────────────────────────────────────────────
    acc = spec.get("accessory", "none")
    
    if acc == "cyber_headset":
        # Zeta-style sleek cyber earset on viewer's right ear
        draw.ellipse([176, 112, 196, 136], fill=(20, 24, 33, 255), outline=accent_col, width=2)
        draw.ellipse([182, 118, 190, 130], fill=accent_col)
        # Mic boom
        draw.line([(182, 128), (164, 142)], fill=(200, 210, 225, 255), width=2)
        draw.rectangle([160, 140, 164, 144], fill=accent_col)
        
    elif acc == "cyber_visor":
        # Daiki high-tech tactical HUD monocle
        draw.polygon([(86, 108), (118, 108), (116, 126), (90, 126)], fill=(20, 184, 166, 90)) # Cyan glass
        draw.line([(86, 108), (118, 108), (116, 126), (90, 126), (86, 108)], fill=accent_col, width=2)
        draw.line([(84, 112), (76, 118)], fill=accent_col, width=2) # Ear frame
        # Target reticle
        draw.line([(100, 113), (104, 113)], fill=(255, 255, 255, 220), width=1)
        draw.line([(102, 111), (102, 115)], fill=(255, 255, 255, 220), width=1)
        
    elif acc == "chic_glasses":
        # Hana stylish thin crimson-rimmed anime glasses
        draw.rectangle([88, 110, 116, 126], outline=accent_col, width=2)
        draw.rectangle([140, 110, 168, 126], outline=accent_col, width=2)
        draw.line([(116, 116), (140, 116)], fill=accent_col, width=2)
        # Lens glint
        draw.line([(92, 112), (98, 124)], fill=(255, 255, 255, 140), width=1)
        draw.line([(144, 112), (150, 124)], fill=(255, 255, 255, 140), width=1)
        
    elif acc == "cyber_cat_ears":
        # Sora cute cat-ear shaped headset antenna
        draw.polygon([(78, 40), (66, 15), (94, 30)], fill=(30, 41, 59, 255))
        draw.polygon([(80, 36), (72, 22), (90, 30)], fill=accent_col)
        draw.polygon([(178, 40), (190, 15), (162, 30)], fill=(30, 41, 59, 255))
        draw.polygon([(176, 36), (184, 22), (166, 30)], fill=accent_col)
        # Headband arc
        draw.arc([74, 18, 182, 60], start=190, end=350, fill=(30, 41, 59, 255), width=3)
        
    elif acc == "cute_hairpins":
        # Yuki cute cyber sakura hairpins
        draw.rectangle([78, 80, 88, 84], fill=accent_col)
        draw.rectangle([80, 88, 90, 92], fill=(255, 255, 255, 255))

    # Composite character onto background card
    final_img = Image.alpha_composite(bg_img, char_img)
    return final_img

# ── DEFINITIONS FOR ALL 9 TARGET AVATARS ────────────────────────────────────
AVATAR_CONFIGS = [
    # ── 6 AUTHENTIC ANIME BOYS ──
    {
        "id": "boy_cheerful",
        "gender": "male",
        "bg_color": (230, 245, 255, 255), # Pastel sky blue
        "border_color": (255, 255, 255, 255),
        "accent_color": (14, 165, 233, 255), # Sky blue
        "hair_color": (180, 105, 50, 255), # Warm spiky brown
        "eye_color": (245, 158, 11, 255),  # Amber gold
        "hair_style": "boy_spiky",
        "expression": "cheerful",
        "accessory": "cyber_headset",
    },
    {
        "id": "boy_cool",
        "gender": "male",
        "bg_color": (215, 245, 242, 255), # Pastel mint/cyan
        "border_color": (255, 255, 255, 255),
        "accent_color": (20, 184, 166, 255), # Teal
        "hair_color": (28, 42, 68, 255),   # Deep navy blue
        "eye_color": (45, 212, 191, 255),  # Cyan-teal
        "hair_style": "boy_cool_bangs",
        "expression": "cool",
        "accessory": "cyber_headset",
    },
    {
        "id": "boy_smug",
        "gender": "male",
        "bg_color": (240, 230, 255, 255), # Pastel lavender
        "border_color": (255, 255, 255, 255),
        "accent_color": (168, 85, 247, 255), # Indigo purple
        "hair_color": (215, 222, 232, 255), # Silver ash
        "eye_color": (147, 51, 234, 255),  # Violet
        "hair_style": "boy_messy_silver",
        "expression": "smug",
        "accessory": "cyber_headset",
    },
    {
        "id": "boy_serious",
        "gender": "male",
        "bg_color": (220, 240, 245, 255), # Slate mint
        "border_color": (255, 255, 255, 255),
        "accent_color": (16, 185, 129, 255), # Emerald
        "hair_color": (22, 30, 46, 255),   # Dark charcoal
        "eye_color": (59, 130, 246, 255),  # Cobalt blue
        "hair_style": "boy_slicked_visor",
        "expression": "serious",
        "accessory": "cyber_visor",
    },
    {
        "id": "boy_pouty",
        "gender": "male",
        "bg_color": (255, 238, 230, 255), # Peach cream
        "border_color": (255, 255, 255, 255),
        "accent_color": (251, 146, 60, 255), # Amber orange
        "hair_color": (245, 190, 110, 255), # Soft blonde
        "eye_color": (217, 119, 6, 255),   # Hazel brown
        "hair_style": "boy_fluffy_pout",
        "expression": "pouty",
        "accessory": "cyber_cat_ears",
    },
    {
        "id": "boy_fierce",
        "gender": "male",
        "bg_color": (255, 230, 230, 255), # Soft coral
        "border_color": (255, 255, 255, 255),
        "accent_color": (239, 68, 68, 255), # Crimson
        "hair_color": (185, 28, 28, 255),   # Fiery red
        "eye_color": (220, 38, 38, 255),   # Crimson
        "hair_style": "boy_fierce_up",
        "expression": "fierce",
        "accessory": "cyber_headset",
    },

    # ── 3 CLEAN ANIME GIRLS (REPLACING GLITCHED SPRITES) ──
    {
        "id": "girl_serious",
        "gender": "female",
        "bg_color": (235, 230, 250, 255), # Lavender
        "border_color": (255, 255, 255, 255),
        "accent_color": (220, 38, 38, 255), # Red glasses frame
        "hair_color": (35, 25, 55, 255),   # Deep violet-black
        "eye_color": (168, 85, 247, 255),  # Purple
        "hair_style": "girl_sleek_dark",
        "expression": "serious",
        "accessory": "chic_glasses",
    },
    {
        "id": "girl_pouty",
        "gender": "female",
        "bg_color": (255, 225, 235, 255), # Rose pink
        "border_color": (255, 255, 255, 255),
        "accent_color": (244, 114, 182, 255), # Pink
        "hair_color": (251, 146, 120, 255), # Pastel peach-pink
        "eye_color": (236, 72, 153, 255),  # Deep pink
        "hair_style": "girl_wavy_pink",
        "expression": "pouty",
        "accessory": "cute_hairpins",
    },
    {
        "id": "girl_fierce",
        "gender": "female",
        "bg_color": (255, 228, 222, 255), # Coral
        "border_color": (255, 255, 255, 255),
        "accent_color": (239, 68, 68, 255), # Red
        "hair_color": (195, 35, 35, 255),   # Crimson
        "eye_color": (248, 113, 113, 255), # Ruby red
        "hair_style": "girl_fierce_tails",
        "expression": "fierce",
        "accessory": "cyber_headset",
    },
]

def main():
    print("Building authentic 16-bit retro anime avatars...")
    for cfg in AVATAR_CONFIGS:
        img = build_character_avatar(cfg)
        out_name = f"{cfg['id']}.png"
        out_path = os.path.join(OUTPUT_DIR, out_name)
        img.save(out_path, "PNG")
        print(f"✓ Saved: {out_path} ({img.size[0]}x{img.size[1]}) - Gender: {cfg['gender']}")

    print("All avatars generated cleanly with zero glitched boxes!")

if __name__ == "__main__":
    main()
