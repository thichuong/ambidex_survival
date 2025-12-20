#!/usr/bin/env python3
"""
Generate weapon and spell icons for Ambidex Survival.
Creates visually appealing geometric icons using PIL.
"""

from PIL import Image, ImageDraw
import math
import os

# Output directory
OUTPUT_DIR = "assets/ui/icons"
ICON_SIZE = 128

def create_icon(draw_func, filename, bg_color=(0, 0, 0, 0)):
    """Create an icon with the given draw function."""
    # Create at 4x size for anti-aliasing then resize
    scale = 4
    size = ICON_SIZE * scale
    img = Image.new('RGBA', (size, size), bg_color)
    draw = ImageDraw.Draw(img)
    draw_func(draw, size)
    
    # Resize down with high quality resampling
    img = img.resize((ICON_SIZE, ICON_SIZE), Image.Resampling.LANCZOS)
    img.save(os.path.join(OUTPUT_DIR, filename))
    print(f"Created: {filename}")

def draw_shuriken(draw, size):
    """Draw a 4-pointed shuriken/ninja star."""
    center = size // 2
    
    # Draw shadow first
    shadow_offset = size * 0.02
    points_shadow = []
    for i in range(8):
        angle = math.radians(i * 45 - 22.5)
        r = size * 0.45 if i % 2 == 0 else size * 0.15
        x = center + shadow_offset + r * math.cos(angle)
        y = center + shadow_offset + r * math.sin(angle)
        points_shadow.append((x, y))
    draw.polygon(points_shadow, fill=(0, 0, 0, 100))

    # Main blade with gradient-like effect (simulated by drawing halves)
    for i in range(4):
        # Each arm of the star
        angle_base = i * 90 - 22.5
        
        # We draw each "blade" as two triangles to simulate lighting
        # Light side
        points_light = [
            (center, center),
            (center + size * 0.45 * math.cos(math.radians(angle_base)), center + size * 0.45 * math.sin(math.radians(angle_base))),
            (center + size * 0.15 * math.cos(math.radians(angle_base + 45)), center + size * 0.15 * math.sin(math.radians(angle_base + 45)))
        ]
        draw.polygon(points_light, fill=(0, 240, 240, 255))
        
        # Dark side
        points_dark = [
            (center, center),
            (center + size * 0.45 * math.cos(math.radians(angle_base)), center + size * 0.45 * math.sin(math.radians(angle_base))),
            (center + size * 0.15 * math.cos(math.radians(angle_base - 45)), center + size * 0.15 * math.sin(math.radians(angle_base - 45)))
        ]
        draw.polygon(points_dark, fill=(0, 180, 180, 255))
        
    # Center circle/bearing
    r = size * 0.08
    draw.ellipse([center-r, center-r, center+r, center+r], fill=(200, 255, 255, 255), outline=(0, 100, 100, 255), width=int(size*0.01))

def draw_sword_normal(draw, size):
    """Draw a complete sword."""
    center = size // 2
    
    # Dimensions
    blade_width = size * 0.12
    blade_h = size * 0.6
    guard_w = size * 0.35
    guard_h = size * 0.04
    handle_h = size * 0.2
    
    # Shadow
    off = size * 0.02
    draw.polygon([
        (center + off, size * 0.1 + off), 
        (center + blade_width/2 + off, size * 0.2 + off),
        (center + blade_width/2 + off, size * 0.7 + off),
        (center - blade_width/2 + off, size * 0.7 + off),
        (center - blade_width/2 + off, size * 0.2 + off)
    ], fill=(0,0,0,80))

    # Blade (Base)
    draw.polygon([
        (center, size * 0.1), 
        (center + blade_width/2, size * 0.2), # Shoulder right
        (center + blade_width/2, size * 0.7), # Base right
        (center - blade_width/2, size * 0.7), # Base left
        (center - blade_width/2, size * 0.2)  # Shoulder left
    ], fill=(200, 200, 220, 255))
    
    # Blade ridge (Light half)
    draw.polygon([
        (center, size * 0.1),
        (center, size * 0.7),
        (center - blade_width/2, size * 0.7),
        (center - blade_width/2, size * 0.2)
    ], fill=(230, 230, 250, 255))
    
    # Blade ridge (Dark half)
    draw.polygon([
        (center, size * 0.1),
        (center + blade_width/2, size * 0.2),
        (center + blade_width/2, size * 0.7),
        (center, size * 0.7)
    ], fill=(160, 160, 190, 255))
    
    # Guard
    draw.rectangle([
        center - guard_w/2, size * 0.68,
        center + guard_w/2, size * 0.74
    ], fill=(218, 165, 32, 255), outline=(100, 80, 0, 255), width=int(size*0.005))
    
    # Handle
    draw.rectangle([
        center - blade_width/4, size * 0.74,
        center + blade_width/4, size * 0.92
    ], fill=(139, 69, 19, 255))
    
    # Handle grip texture
    for i in range(3):
        y = size * (0.78 + i * 0.04)
        draw.line([center - blade_width/4, y, center + blade_width/4, y], fill=(100, 50, 10, 255), width=int(size*0.005))
        
    # Pommel
    pommel_r = size * 0.05
    draw.ellipse([
        center - pommel_r, size * 0.9,
        center + pommel_r, size * 0.9 + pommel_r*2
    ], fill=(218, 165, 32, 255), outline=(100, 80, 0, 255), width=int(size*0.005))

def draw_sword_shattered(draw, size):
    """Draw a shattered/broken sword with fragments."""
    center = size // 2
    blade_width = size * 0.12
    
    # Main broken blade stub
    draw.polygon([
        (center - blade_width/2, size * 0.7),
        (center + blade_width/2, size * 0.7),
        (center + blade_width/2, size * 0.5), # jagged break
        (center, size * 0.45),
        (center - blade_width/4, size * 0.55),
        (center - blade_width/2, size * 0.4)
    ], fill=(180, 180, 220, 255), outline=(100, 100, 120, 255), width=int(size*0.005))
    
    # Fragments
    fragments = [
        (center - size*0.1, size * 0.2, size*0.08, 15),
        (center + size*0.12, size * 0.25, size*0.06, -20),
        (center - size*0.05, size * 0.1, size*0.05, 45),
        (center + size*0.05, size * 0.35, size*0.04, 80)
    ]
    
    for x, y, s, angle in fragments:
        # Simple triangle/quad fragments rotated (approximated)
        draw.polygon([
            (x, y-s/2), (x+s/2, y+s/2), (x-s/2, y+s/2)
        ], fill=(200, 200, 255, 255), outline=(255, 255, 255, 150), width=int(size*0.003))

    # Energy Glow connecting them
    draw.line([(center, size*0.45), (center - size*0.1, size * 0.2)], fill=(100, 200, 255, 100), width=int(size*0.01))
    draw.line([(center, size*0.45), (center + size*0.12, size * 0.25)], fill=(100, 200, 255, 100), width=int(size*0.01))
    
    # Guard (Darker/Worn)
    guard_w = size * 0.35
    draw.rectangle([
        center - guard_w/2, size * 0.68,
        center + guard_w/2, size * 0.74
    ], fill=(184, 134, 11, 255), outline=(80, 60, 0, 255), width=int(size*0.005))
    
    # Handle
    draw.rectangle([
        center - blade_width/4, size * 0.74,
        center + blade_width/4, size * 0.92
    ], fill=(101, 67, 33, 255))
    
    # Pommel
    pommel_r = size * 0.05
    draw.ellipse([
        center - pommel_r, size * 0.9,
        center + pommel_r, size * 0.9 + pommel_r*2
    ], fill=(184, 134, 11, 255), outline=(80, 60, 0, 255), width=int(size*0.005))

def draw_gun_single(draw, size):
    """Draw a pistol icon."""
    center = size // 2
    
    # Body
    barrel_l = size * 0.2
    barrel_r = size * 0.8
    barrel_top = size * 0.35
    barrel_bot = size * 0.5
    
    # Grip
    grip_l = size * 0.25
    grip_r = size * 0.4
    grip_bot = size * 0.8
    
    # Draw grip first
    draw.polygon([
        (grip_l, barrel_bot), (grip_r, barrel_bot),
        (grip_r + size*0.05, grip_bot), (grip_l - size*0.05, grip_bot)
    ], fill=(100, 80, 60, 255), outline=(60, 40, 20, 255), width=int(size*0.005))
    
    # Barrel/Slide
    draw.rectangle([barrel_l, barrel_top, barrel_r, barrel_bot], 
                   fill=(80, 80, 90, 255), outline=(40, 40, 50, 255), width=int(size*0.005))
    
    # Detail: Ejection port
    draw.rectangle([center, barrel_top + size*0.02, center + size*0.1, barrel_top + size*0.08], fill=(40, 40, 40, 255))
    
    # Muzzle flash hint or bullet
    bullet_r = size * 0.02
    draw.ellipse([barrel_r + size*0.05, (barrel_top+barrel_bot)/2 - bullet_r, 
                  barrel_r + size*0.05 + bullet_r*2, (barrel_top+barrel_bot)/2 + bullet_r],
                  fill=(255, 215, 0, 255))

def draw_gun_shotgun(draw, size):
    """Draw a shotgun."""
    center = size // 2
    
    barrel_l = size * 0.15
    barrel_r = size * 0.85
    barrel_top = size * 0.4
    
    # Stock
    draw.polygon([
        (barrel_l, size * 0.45),
        (barrel_l - size*0.1, size * 0.6),
        (barrel_l - size*0.1, size * 0.7),
        (barrel_l + size*0.2, size * 0.6)
    ], fill=(139, 69, 19, 255))
    
    # Dual Barrels
    for i in range(2):
        y = barrel_top + i * size * 0.1
        draw.rectangle([barrel_l, y, barrel_r, y + size * 0.08], 
                       fill=(60, 60, 70, 255), outline=(30, 30, 30, 255), width=int(size*0.004))
    
    # Pump handle
    draw.rectangle([barrel_l + size*0.1, size*0.6, barrel_l + size*0.35, size*0.65], 
                   fill=(100, 70, 30, 255))
                   
    # Shells flying
    draw.rectangle([barrel_r + 5, barrel_top, barrel_r + 15, barrel_top+10], fill=(200, 0, 0, 255))
    draw.rectangle([barrel_r + 10, barrel_top+20, barrel_r + 20, barrel_top+30], fill=(200, 0, 0, 255))

def draw_gun_rapid(draw, size):
    """Draw a machine gun."""
    center = size // 2
    barrel_l = size * 0.15
    barrel_r = size * 0.8
    
    # Main body
    draw.rectangle([barrel_l, size*0.35, barrel_l + size*0.4, size*0.55], fill=(50, 50, 50, 255))
    
    # Multiple barrels (Gatling style ish)
    draw.rectangle([barrel_l + size*0.4, size*0.38, barrel_r, size*0.42], fill=(80, 80, 80, 255))
    draw.rectangle([barrel_l + size*0.4, size*0.48, barrel_r, size*0.52], fill=(80, 80, 80, 255))
    
    # Ammo box / Magazine
    draw.rectangle([barrel_l + size*0.2, size*0.55, barrel_l + size*0.35, size*0.75], 
                   fill=(30, 40, 30, 255))
                   
    # Lots of bullets/motion
    for i in range(3):
        x = barrel_r + 10 + i * 20
        y = size * 0.45 + (i%2) * 10 - 5
        draw.line([x, y, x+15, y], fill=(255, 200, 50, 200), width=int(size*0.01))

def draw_magic_bolt(draw, size):
    """Draw a detailed lightning bolt."""
    center = size // 2
    
    points = [
        (center + size*0.2, size*0.1),
        (center - size*0.1, size*0.45),
        (center + size*0.1, size*0.45),
        (center - size*0.2, size*0.9),
    ]
    
    # Outer Glow
    draw.line(points, fill=(180, 0, 255, 100), width=int(size*0.15), joint='curve')
    draw.line(points, fill=(200, 100, 255, 180), width=int(size*0.08), joint='curve')
    
    # Core
    draw.line(points, fill=(255, 240, 255, 255), width=int(size*0.03), joint='curve')

def draw_magic_laser(draw, size):
    """Draw a intense laser beam."""
    center = size // 2
    
    # Outer glow (cyan)
    draw.line([(size*0.1, center), (size*0.9, center)], fill=(0, 255, 255, 50), width=int(size*0.2))
    draw.line([(size*0.1, center), (size*0.9, center)], fill=(0, 255, 255, 100), width=int(size*0.1))
    
    # Inner beam (white/bright cyan)
    draw.line([(size*0.1, center), (size*0.9, center)], fill=(200, 255, 255, 255), width=int(size*0.04))
    
    # Particles/sparks along the beam
    for i in range(5):
        w = size * 0.02
        x = size * (0.2 + i * 0.15)
        y = center + (i % 2 * 2 - 1) * size * 0.03
        draw.ellipse([x-w, y-w, x+w, y+w], fill=(255, 255, 255, 200))

def draw_magic_nova(draw, size):
    """Draw a radial explosion."""
    center = size // 2
    
    # Expanding rings
    for r_fac, alpha in [(0.45, 50), (0.35, 100), (0.25, 200)]:
        r = size * r_fac
        draw.ellipse([center-r, center-r, center+r, center+r], outline=(255, 100, 255, alpha), width=int(size*0.02))
        
    # Core
    r = size * 0.15
    draw.ellipse([center-r, center-r, center+r, center+r], fill=(255, 200, 255, 255))
    
    # Rays
    for i in range(0, 360, 30):
        length = size * 0.4
        rad = math.radians(i)
        draw.line([
            (center + size*0.15*math.cos(rad), center + size*0.15*math.sin(rad)),
            (center + length*math.cos(rad), center + length*math.sin(rad))
        ], fill=(255, 150, 255, 200), width=int(size*0.01))

def draw_magic_blink(draw, size):
    """Draw a teleport motion trail."""
    center = size // 2
    
    # Phantom outlines
    for i in range(3):
        alpha = 50 + i * 60
        x_off = (i-1) * size * 0.25
        
        # Silhouette
        body_w = size * 0.1
        body_h = size * 0.25
        draw.ellipse([center+x_off-body_w/2, center-body_h, center+x_off+body_w/2, center-body_h+body_w], fill=(200, 255, 255, alpha))
        draw.polygon([
            (center+x_off, center-body_h+body_w),
            (center+x_off-body_w, center+body_h),
            (center+x_off+body_w, center+body_h)
        ], fill=(200, 255, 255, alpha))
        
    # Dash lines
    draw.line([(size*0.1, center+size*0.1), (size*0.9, center+size*0.1)], fill=(150, 255, 255, 100), width=int(size*0.01))

def draw_magic_global(draw, size):
    """Draw a planetary/global effect."""
    center = size // 2
    r = size * 0.35
    
    # Planet body (gradient simulation)
    draw.ellipse([center-r, center-r, center+r, center+r], fill=(0, 100, 200, 255))
    draw.ellipse([center-r*0.8, center-r*0.8, center+r*0.4, center+r*0.4], fill=(50, 150, 250, 100)) # Highlight
    
    # Grid lines/Lat-Long
    draw.ellipse([center-r, center-r*0.4, center+r, center+r*0.4], outline=(100, 200, 255, 150), width=int(size*0.005))
    draw.ellipse([center-r*0.4, center-r, center+r*0.4, center+r], outline=(100, 200, 255, 150), width=int(size*0.005))
    
    # Orbiting satellites/energy
    draw.arc([center-r*1.2, center-r*1.2, center+r*1.2, center+r*1.2], 0, 180, fill=(255, 255, 200, 200), width=int(size*0.01))
    draw.arc([center-r*1.2, center-r*1.2, center+r*1.2, center+r*1.2], 180, 240, fill=(255, 255, 200, 200), width=int(size*0.01))

def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    print("Generating improved icons...")
    
    # Weapons
    create_icon(draw_shuriken, "shuriken.png")
    create_icon(draw_sword_normal, "sword_normal.png")
    create_icon(draw_sword_shattered, "sword_shattered.png")
    create_icon(draw_gun_single, "gun_single.png")
    create_icon(draw_gun_shotgun, "gun_shotgun.png")
    create_icon(draw_gun_rapid, "gun_rapid.png")
    
    # Magic
    create_icon(draw_magic_bolt, "magic_bolt.png")
    create_icon(draw_magic_laser, "magic_laser.png")
    create_icon(draw_magic_nova, "magic_nova.png")
    create_icon(draw_magic_blink, "magic_blink.png")
    create_icon(draw_magic_global, "magic_global.png")
    
    print(f"Done! Check {OUTPUT_DIR}")

if __name__ == "__main__":
    main()
