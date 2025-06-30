# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository contains ClaudeTetris - a PC-optimized web-based Tetris game with innovative point systems and fever mode. The project has two main components:

1. **Current Implementation**: `tetris/` - A fully functional HTML/CSS/JavaScript Tetris game (85% complete)
2. **Future Architecture**: `doc/` - Planning documents for a React/Next.js version

## Development Commands

### Current HTML/CSS/JS Version (tetris/)
```bash
# Start development server
cd tetris
python -m http.server 8000
# or
npx serve .

# Open browser to http://localhost:8000
```

### Testing
- Manual testing in browsers (Chrome, Firefox, Safari, Edge)
- No automated test framework currently implemented

## Current Codebase Architecture

### File Structure (tetris/)
```
tetris/
├── index.html              # Main game page
├── style.css               # Game styling and animations
├── js/
│   ├── script.js           # Main game engine and logic
│   ├── onlineAchievementSystem.js  # Achievement system
│   └── realtimeRankingSystem.js    # Ranking system
├── src/                    # Modular architecture (preferred)
│   ├── core/
│   │   ├── gameEngine.js   # Core game mechanics
│   │   ├── gameField.js    # Game field management
│   │   └── tetrominos.js   # Tetromino definitions
│   ├── systems/
│   │   ├── achievementSystem.js    # Achievement tracking
│   │   ├── feverMode.js           # Fever mode mechanics
│   │   ├── pointSystem.js         # Point & exchange system
│   │   └── scoreManager.js        # Score & ranking management
│   ├── tutorial/
│   │   ├── tutorialSystem.js      # Tutorial logic
│   │   └── tutorialUI.js          # Tutorial interface
│   └── ui/
│       └── uiManager.js           # UI state management
└── log/                    # Development logs (YYYYMMDD_HHmm_log.md format)
```

### Core Game Systems

#### Point & Exchange System
- Points earned: Block placement (10P), drops (0.5-1P per row)
- Exchange costs: Cumulative 45P → 65P → 90P → 120P → 160P
- Resets when new tetromino is placed
- Free exchanges during fever mode

#### Fever Mode System
- Triggered: After 20 block placements
- Duration: 30 seconds
- Effects: 4x score multiplier, unlimited free exchanges
- Visual effects with GPU acceleration optimization

#### Multi-Spin System
- T-Spin: 2000/5000/10000 points (Single/Double/Triple)
- SZ-Spin: 800/2000/4000 points (requires wall kick)
- I-Spin: 600/1500/3000 points (requires wall kick)
- JL-Spin: 700/1800/3500 points (requires wall kick)

#### Achievement System
- 15 achievements across categories: Basic, Score, Technical, Challenge, Special
- Progress tracking and point rewards
- Real-time unlock notifications

### Technical Implementation

#### Performance Optimizations
- GPU acceleration using `will-change` and `translate3d()`
- Reduced particle count (20 → 8) for fever mode
- 60fps target with requestAnimationFrame
- Memory management with proper cleanup

#### PC-Optimized Design
- Minimum screen width: 1200px
- Layout toggle: Horizontal (3-area) ⟷ Vertical (stacked)
- Keyboard-only controls
- No mobile support (intentionally disabled)

#### Controls
- Arrow keys: Movement and rotation
- Space: Hard drop
- C/V: Hold slots 1/2 (15P cost each)
- E: Exchange next piece
- L: Delete bottom line (200P cost)

## Development Guidelines

### Code Style
- Use existing modular structure in `src/` directory
- Follow JavaScript ES6+ patterns
- GPU acceleration for animations (`will-change`, `translate3d()`)
- localStorage for data persistence

### Performance Requirements
- Maintain 60fps during gameplay
- Optimize fever mode animations
- Use efficient DOM manipulation
- Minimize memory allocations during gameplay

### File Organization
- Prefer modular files in `src/` over monolithic `js/script.js`
- Core game logic in `src/core/`
- Game systems in `src/systems/`
- UI components in `src/ui/`

### Logging
- Create development logs in `log/` directory
- Format: `YYYYMMDD_HHmm_log.md`
- Document implementation changes, fixes, and TODOs

## Current Implementation Status

### ✅ Completed Features
- Core Tetris gameplay (10×20 grid, 7 piece types)
- Point & exchange system with cumulative costs
- Fever mode with 4x multiplier and free exchanges
- Multi-spin detection (T-Spin, SZ-Spin, I-Spin, JL-Spin)
- Achievement system with 15 achievements
- Ranking system with 14 ranks (無段 → 永世名人)
- Tutorial system (Level 1-2 implemented)
- Responsive layout toggle (horizontal/vertical)
- PC optimization with full-screen layout

### ❌ Planned Features
- Sound system (BGM, sound effects, volume control)
- Keyboard customization
- Daily challenge system
- Theme customization
- Replay system
- Advanced tutorial levels (3-6)

## Future Architecture (React/Next.js)

The `doc/` directory contains architectural planning for a future rewrite:
- React + Next.js with TypeScript
- Redux Toolkit for state management
- Material UI components
- PostgreSQL/MySQL database
- JWT authentication
- API-driven architecture

This planning is separate from the current working game and represents future expansion goals.