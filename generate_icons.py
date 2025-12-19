#!/usr/bin/env python3
"""
Generate weapon and spell icons for Ambidex Survival.
Creates simple but visually appealing geometric icons using PIL.
"""

from PIL import Image, ImageDraw
import math
import os

# Output directory
OUTPUT_DIR = "assets/ui/icons"
ICON_SIZE = 128

def create_icon(draw_func, filename, bg_color=(0, 0, 0, 0)):
    """Create an icon with the given draw function."""
    img = Image.new('RGBA', (ICON_SIZE, ICON_SIZE), bg_color)
    draw = ImageDraw.Draw(img)
    draw_func(draw, ICON_SIZE)
    img.save(os.path.join(OUTPUT_DIR, filename))
    print(f"Created: {filename}")

def draw_shuriken(draw, size):
    """Draw a 4-pointed shuriken/ninja star."""
    center = size // 2
    # Outer points
    points = []
    for i in range(8):
        angle = math.radians(i * 45 - 22.5)
        r = size * 0.45 if i % 2 == 0 else size * 0.15
        x = center + r * math.cos(angle)
        y = center + r * math.sin(angle)
        points.append((x, y))
    
    # Cyan color for shuriken
    draw.polygon(points, fill=(0, 220, 220, 255), outline=(255, 255, 255, 255))
    # Center circle
    draw.ellipse([center-8, center-8, center+8, center+8], fill=(100, 255, 255, 255))

def draw_sword_normal(draw, size):
    """Draw a complete sword."""
    center = size // 2
    # Blade (white/silver)
    blade_top = size * 0.1
    blade_bottom = size * 0.65
    blade_width = size * 0.08
    draw.polygon([
        (center, blade_top),  # tip
        (center - blade_width, blade_bottom),
        (center + blade_width, blade_bottom)
    ], fill=(220, 220, 240, 255), outline=(255, 255, 255, 255))
    
    # Guard (golden)
    guard_y = size * 0.65
    guard_width = size * 0.25
    guard_height = size * 0.05
    draw.rectangle([
        center - guard_width, guard_y,
        center + guard_width, guard_y + guard_height
    ], fill=(200, 180, 100, 255), outline=(255, 220, 100, 255))
    
    # Handle (brown)
    handle_top = guard_y + guard_height
    handle_bottom = size * 0.85
    handle_width = size * 0.05
    draw.rectangle([
        center - handle_width, handle_top,
        center + handle_width, handle_bottom
    ], fill=(139, 90, 43, 255), outline=(180, 120, 60, 255))
    
    # Pommel (golden circle)
    pommel_y = size * 0.87
    pommel_r = size * 0.04
    draw.ellipse([
        center - pommel_r, pommel_y,
        center + pommel_r, pommel_y + pommel_r * 2
    ], fill=(200, 180, 100, 255), outline=(255, 220, 100, 255))

def draw_sword_shattered(draw, size):
    """Draw a shattered/broken sword with fragments."""
    center = size // 2
    # Broken blade (shorter)
    blade_top = size * 0.25
    blade_bottom = size * 0.65
    blade_width = size * 0.08
    
    # Main broken blade
    draw.polygon([
        (center - blade_width * 0.5, blade_top),
        (center + blade_width, blade_top + size * 0.05),
        (center - blade_width, blade_bottom),
        (center + blade_width, blade_bottom)
    ], fill=(180, 180, 220, 255), outline=(200, 200, 255, 255))
    
    # Floating fragments
    fragments = [
        (center - 20, size * 0.15, 10),
        (center + 18, size * 0.18, 8),
        (center - 25, size * 0.28, 6),
        (center + 22, size * 0.25, 7),
    ]
    for fx, fy, fs in fragments:
        draw.polygon([
            (fx, fy - fs),
            (fx + fs, fy),
            (fx, fy + fs * 0.5),
            (fx - fs, fy)
        ], fill=(200, 200, 255, 200), outline=(220, 220, 255, 255))
    
    # Guard (golden)
    guard_y = size * 0.65
    guard_width = size * 0.25
    guard_height = size * 0.05
    draw.rectangle([
        center - guard_width, guard_y,
        center + guard_width, guard_y + guard_height
    ], fill=(200, 180, 100, 255), outline=(255, 220, 100, 255))
    
    # Handle
    handle_top = guard_y + guard_height
    handle_bottom = size * 0.85
    handle_width = size * 0.05
    draw.rectangle([
        center - handle_width, handle_top,
        center + handle_width, handle_bottom
    ], fill=(139, 90, 43, 255), outline=(180, 120, 60, 255))
    
    # Pommel
    pommel_y = size * 0.87
    pommel_r = size * 0.04
    draw.ellipse([
        center - pommel_r, pommel_y,
        center + pommel_r, pommel_y + pommel_r * 2
    ], fill=(200, 180, 100, 255), outline=(255, 220, 100, 255))

def draw_gun_single(draw, size):
    """Draw a pistol icon (single shot mode)."""
    center = size // 2
    # Barrel
    barrel_left = size * 0.15
    barrel_right = size * 0.85
    barrel_top = center - size * 0.08
    barrel_bottom = center + size * 0.08
    draw.rectangle([barrel_left, barrel_top, barrel_right, barrel_bottom],
                   fill=(255, 180, 0, 255), outline=(255, 220, 100, 255))
    
    # Muzzle (darker)
    muzzle_width = size * 0.08
    draw.rectangle([barrel_right - muzzle_width, barrel_top, barrel_right, barrel_bottom],
                   fill=(200, 140, 0, 255))
    
    # Handle
    handle_left = center - size * 0.12
    handle_right = center + size * 0.08
    handle_top = barrel_bottom
    handle_bottom = size * 0.82
    draw.polygon([
        (handle_left, handle_top),
        (handle_right, handle_top),
        (handle_right + size * 0.05, handle_bottom),
        (handle_left - size * 0.05, handle_bottom)
    ], fill=(180, 120, 40, 255), outline=(220, 160, 60, 255))
    
    # Single bullet indicator
    bullet_x = barrel_right - size * 0.15
    bullet_y = center
    draw.ellipse([bullet_x - 4, bullet_y - 4, bullet_x + 4, bullet_y + 4],
                 fill=(255, 255, 200, 255))

def draw_gun_shotgun(draw, size):
    """Draw a shotgun icon (multiple barrels)."""
    center = size // 2
    # Three barrels
    for offset in [-12, 0, 12]:
        barrel_left = size * 0.15
        barrel_right = size * 0.85
        barrel_top = center + offset - 5
        barrel_bottom = center + offset + 5
        draw.rectangle([barrel_left, barrel_top, barrel_right, barrel_bottom],
                       fill=(255, 160, 0, 255), outline=(255, 200, 80, 255))
    
    # Handle (shared)
    handle_left = center - size * 0.15
    handle_right = center + size * 0.12
    handle_top = center + 18
    handle_bottom = size * 0.85
    draw.polygon([
        (handle_left, handle_top),
        (handle_right, handle_top),
        (handle_right + size * 0.05, handle_bottom),
        (handle_left - size * 0.05, handle_bottom)
    ], fill=(160, 100, 30, 255), outline=(200, 140, 50, 255))
    
    # Multiple bullet indicators
    for i, offset in enumerate([-12, 0, 12]):
        bullet_x = size * 0.75
        bullet_y = center + offset
        draw.ellipse([bullet_x - 3, bullet_y - 3, bullet_x + 3, bullet_y + 3],
                     fill=(255, 255, 150, 255))

def draw_gun_rapid(draw, size):
    """Draw a rapid-fire gun with speed lines."""
    center = size // 2
    # Main barrel
    barrel_left = size * 0.1
    barrel_right = size * 0.8
    barrel_top = center - size * 0.1
    barrel_bottom = center + size * 0.1
    draw.rectangle([barrel_left, barrel_top, barrel_right, barrel_bottom],
                   fill=(255, 100, 0, 255), outline=(255, 150, 50, 255))
    
    # Speed lines / muzzle flash
    for i in range(3):
        line_x = barrel_right + (i + 1) * 8
        line_len = 20 - i * 5
        draw.line([(line_x, center - line_len), (line_x, center + line_len)],
                  fill=(255, 200, 100, 200 - i * 50), width=3)
    
    # Multiple small bullets flying
    for i in range(4):
        bullet_x = barrel_right + 5 + i * 10
        bullet_y = center + (i % 2 * 2 - 1) * 5
        draw.ellipse([bullet_x - 2, bullet_y - 2, bullet_x + 2, bullet_y + 2],
                     fill=(255, 255, 100, 255))
    
    # Handle
    handle_left = center - size * 0.1
    handle_right = center + size * 0.1
    handle_top = barrel_bottom
    handle_bottom = size * 0.8
    draw.polygon([
        (handle_left, handle_top),
        (handle_right, handle_top),
        (handle_right + 3, handle_bottom),
        (handle_left - 3, handle_bottom)
    ], fill=(140, 80, 20, 255), outline=(180, 120, 40, 255))

def draw_magic_bolt(draw, size):
    """Draw an energy bolt (lightning style)."""
    center = size // 2
    # Lightning bolt shape
    bolt_points = [
        (center + 15, size * 0.1),
        (center - 5, size * 0.45),
        (center + 10, size * 0.45),
        (center - 15, size * 0.9),
        (center + 5, size * 0.55),
        (center - 10, size * 0.55),
    ]
    draw.polygon(bolt_points, fill=(200, 0, 255, 255), outline=(255, 150, 255, 255))
    
    # Glow effect (outer)
    for i in range(1, 4):
        offset_points = [(x + (i * 0.5 if j % 2 == 0 else -i * 0.5), y) 
                         for j, (x, y) in enumerate(bolt_points)]
        draw.polygon(offset_points, outline=(200, 100, 255, 100 - i * 20))

def draw_magic_laser(draw, size):
    """Draw a laser beam icon."""
    center = size // 2
    # Main beam
    beam_left = size * 0.1
    beam_right = size * 0.9
    beam_height = size * 0.12
    draw.rectangle([beam_left, center - beam_height/2, beam_right, center + beam_height/2],
                   fill=(0, 255, 255, 255), outline=(150, 255, 255, 255))
    
    # Core (brighter)
    core_height = beam_height * 0.4
    draw.rectangle([beam_left, center - core_height/2, beam_right, center + core_height/2],
                   fill=(200, 255, 255, 255))
    
    # End glow
    glow_r = size * 0.15
    draw.ellipse([beam_right - glow_r, center - glow_r, beam_right + glow_r, center + glow_r],
                 fill=(100, 255, 255, 100))

def draw_magic_nova(draw, size):
    """Draw an explosion/nova icon."""
    center = size // 2
    # Outer ring
    for i in range(3):
        r = size * (0.45 - i * 0.1)
        alpha = 200 - i * 50
        draw.ellipse([center - r, center - r, center + r, center + r],
                     outline=(255, 0, 255, alpha), width=3 - i)
    
    # Spikes/rays
    for angle in range(0, 360, 45):
        rad = math.radians(angle)
        inner_r = size * 0.2
        outer_r = size * 0.42
        x1 = center + inner_r * math.cos(rad)
        y1 = center + inner_r * math.sin(rad)
        x2 = center + outer_r * math.cos(rad)
        y2 = center + outer_r * math.sin(rad)
        draw.line([(x1, y1), (x2, y2)], fill=(255, 100, 255, 255), width=3)
    
    # Center
    draw.ellipse([center - 12, center - 12, center + 12, center + 12],
                 fill=(255, 200, 255, 255), outline=(255, 255, 255, 255))

def draw_magic_blink(draw, size):
    """Draw a teleport/blink icon (silhouette fading)."""
    center = size // 2
    # Fading silhouettes (left to right)
    for i in range(3):
        offset = (i - 1) * 25
        alpha = 80 + i * 80
        # Simple person shape
        head_y = size * 0.25
        body_bottom = size * 0.75
        width = size * 0.12
        
        x = center + offset
        # Head
        draw.ellipse([x - 8, head_y - 8, x + 8, head_y + 8],
                     fill=(255, 255, 255, alpha))
        # Body
        draw.polygon([
            (x, head_y + 10),
            (x - width, body_bottom),
            (x + width, body_bottom)
        ], fill=(255, 255, 255, alpha))
    
    # Motion lines
    for y_off in [-10, 0, 10]:
        draw.line([(size * 0.1, center + y_off), (size * 0.3, center + y_off)],
                  fill=(200, 200, 255, 150), width=2)

def draw_magic_global(draw, size):
    """Draw a global/world-wide effect icon."""
    center = size // 2
    # Earth-like circle
    earth_r = size * 0.35
    draw.ellipse([center - earth_r, center - earth_r, center + earth_r, center + earth_r],
                 fill=(100, 150, 255, 200), outline=(200, 220, 255, 255))
    
    # Latitude lines
    for i in range(-2, 3):
        y = center + i * earth_r * 0.4
        half_width = math.sqrt(max(0, earth_r**2 - (i * earth_r * 0.4)**2))
        if half_width > 5:
            draw.arc([center - half_width, y - 5, center + half_width, y + 5],
                     0, 360, fill=(200, 220, 255, 200), width=1)
    
    # Longitude line (vertical)
    draw.arc([center - earth_r * 0.3, center - earth_r, 
              center + earth_r * 0.3, center + earth_r],
             0, 360, fill=(200, 220, 255, 200), width=1)
    
    # Outer glow ring
    glow_r = size * 0.45
    draw.ellipse([center - glow_r, center - glow_r, center + glow_r, center + glow_r],
                 outline=(255, 255, 200, 150), width=2)

def main():
    # Create output directory
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    
    print("Generating weapon and spell icons...")
    
    # Shuriken
    create_icon(draw_shuriken, "shuriken.png")
    
    # Sword
    create_icon(draw_sword_normal, "sword_normal.png")
    create_icon(draw_sword_shattered, "sword_shattered.png")
    
    # Gun
    create_icon(draw_gun_single, "gun_single.png")
    create_icon(draw_gun_shotgun, "gun_shotgun.png")
    create_icon(draw_gun_rapid, "gun_rapid.png")
    
    # Magic spells
    create_icon(draw_magic_bolt, "magic_bolt.png")
    create_icon(draw_magic_laser, "magic_laser.png")
    create_icon(draw_magic_nova, "magic_nova.png")
    create_icon(draw_magic_blink, "magic_blink.png")
    create_icon(draw_magic_global, "magic_global.png")
    
    print(f"\nDone! All icons saved to: {OUTPUT_DIR}/")

if __name__ == "__main__":
    main()
