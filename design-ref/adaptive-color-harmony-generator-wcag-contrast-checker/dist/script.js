/**
 * ============================================
 * ADAPTIVE COLOR HARMONY GENERATOR
 * Demonstrates min/max in color calculations,
 * HTML attributes, and accessibility scoring
 * ============================================
 */

class ColorHarmonyGenerator {
	constructor() {
		// Cache DOM elements
		this.elements = {
			// Inputs
			baseColorPicker: document.getElementById("baseColorPicker"),
			saturationRange: document.getElementById("saturationRange"),
			lightnessRange: document.getElementById("lightnessRange"),
			angleRange: document.getElementById("angleRange"),

			// Value displays
			hexValue: document.getElementById("hexValue"),
			hslValue: document.getElementById("hslValue"),
			saturationValue: document.getElementById("saturationValue"),
			lightnessValue: document.getElementById("lightnessValue"),
			angleValue: document.getElementById("angleValue"),

			// Sections
			angleSection: document.getElementById("angleSection"),
			paletteColors: document.getElementById("paletteColors"),
			modeTag: document.getElementById("modeTag"),
			contrastGrid: document.getElementById("contrastGrid"),
			wcagOverall: document.getElementById("wcagOverall"),
			exportCode: document.getElementById("exportCode"),

			// Wheel markers
			primaryMarker: document.getElementById("primaryMarker"),
			markers: [
				document.getElementById("marker1"),
				document.getElementById("marker2"),
				document.getElementById("marker3"),
				document.getElementById("marker4")
			],

			// Buttons
			harmonyBtns: document.querySelectorAll(".harmony-btn"),
			exportTabs: document.querySelectorAll(".export-tab"),
			randomBtn: document.getElementById("randomBtn"),
			exportBtn: document.getElementById("exportBtn"),
			copyCodeBtn: document.getElementById("copyCodeBtn"),
			copyBtns: document.querySelectorAll(".copy-btn"),

			// Toast
			toast: document.getElementById("toast")
		};

		// State with min/max boundaries
		this.state = {
			baseHue: 239,
			saturation: { value: 80, min: 10, max: 100 },
			lightness: { value: 60, min: 15, max: 90 },
			angle: { value: 30, min: 15, max: 60 },
			harmonyMode: "analogous",
			exportFormat: "css",
			palette: []
		};

		// Harmony mode configurations
		this.harmonyConfigs = {
			complementary: { angles: [180], showAngle: false },
			analogous: { angles: [-30, 30], showAngle: true },
			triadic: { angles: [120, 240], showAngle: false },
			tetradic: { angles: [90, 180, 270], showAngle: false },
			split: { angles: [150, 210], showAngle: true },
			monochromatic: { angles: [], showAngle: false }
		};

		// Color names database (simplified)
		this.colorNames = {
			0: "Red",
			15: "Orange Red",
			30: "Orange",
			45: "Gold",
			60: "Yellow",
			75: "Lime Yellow",
			90: "Lime",
			105: "Spring Green",
			120: "Green",
			135: "Mint",
			150: "Aquamarine",
			165: "Turquoise",
			180: "Cyan",
			195: "Sky Blue",
			210: "Azure",
			225: "Cerulean",
			240: "Blue",
			255: "Indigo",
			270: "Violet",
			285: "Purple",
			300: "Magenta",
			315: "Pink",
			330: "Rose",
			345: "Crimson"
		};

		this.init();
	}

	init() {
		this.bindEvents();
		this.parseInitialColor();
		this.generatePalette();
	}

	bindEvents() {
		// Color picker
		this.elements.baseColorPicker.addEventListener("input", (e) => {
			this.handleColorChange(e.target.value);
		});

		// Range inputs with min/max HTML attributes
		this.elements.saturationRange.addEventListener("input", (e) => {
			this.updateSaturation(e.target.value);
		});

		this.elements.lightnessRange.addEventListener("input", (e) => {
			this.updateLightness(e.target.value);
		});

		this.elements.angleRange.addEventListener("input", (e) => {
			this.updateAngle(e.target.value);
		});

		// Harmony mode buttons
		this.elements.harmonyBtns.forEach((btn) => {
			btn.addEventListener("click", () => {
				this.setHarmonyMode(btn.dataset.harmony);
			});
		});

		// Export format tabs
		this.elements.exportTabs.forEach((tab) => {
			tab.addEventListener("click", () => {
				this.setExportFormat(tab.dataset.format);
			});
		});

		// Action buttons
		this.elements.randomBtn.addEventListener("click", () => this.randomize());
		this.elements.exportBtn.addEventListener("click", () =>
			this.downloadPalette()
		);
		this.elements.copyCodeBtn.addEventListener("click", () => this.copyCode());

		// Copy buttons
		this.elements.copyBtns.forEach((btn) => {
			btn.addEventListener("click", () => this.copyColorValue(btn.dataset.copy));
		});

		// Color card clicks (delegated)
		this.elements.paletteColors.addEventListener("click", (e) => {
			const card = e.target.closest(".color-card");
			if (card) this.copyCardColor(card);
		});
	}

	/**
	 * Parse initial color from picker
	 */
	parseInitialColor() {
		const hex = this.elements.baseColorPicker.value;
		const hsl = this.hexToHSL(hex);
		this.state.baseHue = hsl.h;
		this.state.saturation.value = hsl.s;
		this.state.lightness.value = hsl.l;

		// Update range inputs
		this.elements.saturationRange.value = hsl.s;
		this.elements.lightnessRange.value = hsl.l;
	}

	/**
	 * Handle base color change
	 */
	handleColorChange(hex) {
		const hsl = this.hexToHSL(hex);
		this.state.baseHue = hsl.h;

		// Apply min/max constraints using Math.min/Math.max
		this.state.saturation.value = Math.max(
			this.state.saturation.min,
			Math.min(this.state.saturation.max, hsl.s)
		);

		this.state.lightness.value = Math.max(
			this.state.lightness.min,
			Math.min(this.state.lightness.max, hsl.l)
		);

		this.updateUI();
		this.generatePalette();
	}

	/**
	 * Update saturation with min/max constraints
	 */
	updateSaturation(value) {
		const num = parseInt(value);
		// Apply constraints using Math.min and Math.max
		this.state.saturation.value = Math.max(
			this.state.saturation.min,
			Math.min(this.state.saturation.max, num)
		);

		this.elements.saturationValue.textContent = `${this.state.saturation.value}%`;
		this.updateBaseColorPicker();
		this.generatePalette();
	}

	/**
	 * Update lightness with min/max constraints
	 */
	updateLightness(value) {
		const num = parseInt(value);
		this.state.lightness.value = Math.max(
			this.state.lightness.min,
			Math.min(this.state.lightness.max, num)
		);

		this.elements.lightnessValue.textContent = `${this.state.lightness.value}%`;
		this.updateBaseColorPicker();
		this.generatePalette();
	}

	/**
	 * Update harmony angle with min/max constraints
	 */
	updateAngle(value) {
		const num = parseInt(value);
		this.state.angle.value = Math.max(
			this.state.angle.min,
			Math.min(this.state.angle.max, num)
		);

		this.elements.angleValue.textContent = `${this.state.angle.value}°`;
		this.generatePalette();
	}

	/**
	 * Set harmony mode
	 */
	setHarmonyMode(mode) {
		this.state.harmonyMode = mode;

		// Update button states
		this.elements.harmonyBtns.forEach((btn) => {
			btn.classList.toggle("active", btn.dataset.harmony === mode);
		});

		// Show/hide angle control
		const config = this.harmonyConfigs[mode];
		this.elements.angleSection.style.display = config.showAngle
			? "block"
			: "none";

		// Update mode tag
		this.elements.modeTag.textContent = this.formatModeName(mode);

		this.generatePalette();
	}

	/**
	 * Set export format
	 */
	setExportFormat(format) {
		this.state.exportFormat = format;

		this.elements.exportTabs.forEach((tab) => {
			tab.classList.toggle("active", tab.dataset.format === format);
		});

		this.updateExportCode();
	}

	/**
	 * Generate color palette based on harmony mode
	 * Uses min/max for hue wrapping and color adjustments
	 */
	generatePalette() {
		const { baseHue, saturation, lightness, angle, harmonyMode } = this.state;
		const config = this.harmonyConfigs[harmonyMode];

		// Start with base color
		const palette = [
			{
				hue: baseHue,
				saturation: saturation.value,
				lightness: lightness.value,
				isPrimary: true
			}
		];

		if (harmonyMode === "monochromatic") {
			// Generate lightness variations with min/max constraints
			const variations = [
				{ l: Math.max(lightness.min, lightness.value - 30) },
				{ l: Math.max(lightness.min, lightness.value - 15) },
				{ l: Math.min(lightness.max, lightness.value + 15) },
				{ l: Math.min(lightness.max, lightness.value + 30) }
			];

			variations.forEach((v) => {
				palette.push({
					hue: baseHue,
					saturation: saturation.value,
					lightness: v.l,
					isPrimary: false
				});
			});
		} else {
			// Generate harmony colors based on angles
			let angles = [...config.angles];

			// For analogous and split-complementary, use the custom angle
			if (harmonyMode === "analogous") {
				angles = [-angle.value, angle.value];
			} else if (harmonyMode === "split") {
				angles = [180 - angle.value, 180 + angle.value];
			}

			angles.forEach((a) => {
				// Wrap hue using modulo (keeping within 0-360)
				let newHue = (baseHue + a) % 360;
				if (newHue < 0) newHue += 360;

				palette.push({
					hue: newHue,
					saturation: saturation.value,
					lightness: lightness.value,
					isPrimary: false
				});
			});
		}

		// Convert to full color objects with HEX values
		this.state.palette = palette.map((color, index) => ({
			...color,
			hex: this.hslToHex(color.hue, color.saturation, color.lightness),
			hsl: `hsl(${Math.round(color.hue)}, ${color.saturation}%, ${
				color.lightness
			}%)`,
			name: this.getColorName(color.hue),
			index
		}));

		this.renderPalette();
		this.updateWheelMarkers();
		this.checkAccessibility();
		this.updateExportCode();
		this.updateUI();
	}

	/**
	 * Render palette colors to DOM
	 */
	renderPalette() {
		const { palette } = this.state;

		this.elements.paletteColors.innerHTML = palette
			.map(
				(color, i) => `
          <div class="color-card ${
											color.isPrimary ? "primary" : ""
										}" data-index="${i}">
            <div class="color-swatch" style="background: ${
													color.hex
												}; color: ${this.getContrastColor(color.hex)}">
              <span class="swatch-label">${
															color.isPrimary ? "Primary" : `Color ${i}`
														}</span>
            </div>
            <div class="color-details">
              <span class="color-hex">${color.hex.toUpperCase()}</span>
              <span class="color-name">${color.name}</span>
            </div>
          </div>
        `
			)
			.join("");
	}

	/**
	 * Update color wheel markers
	 */
	updateWheelMarkers() {
		const { palette } = this.state;
		const wheelRadius = 100; // Half of wheel size
		const markerRadius = 77; // Position on the ring

		palette.forEach((color, index) => {
			const marker =
				index === 0
					? this.elements.primaryMarker
					: this.elements.markers[index - 1];

			if (marker) {
				// Convert hue to radians (starting from top)
				const angleRad = (color.hue - 90) * (Math.PI / 180);
				const x = wheelRadius + markerRadius * Math.cos(angleRad);
				const y = wheelRadius + markerRadius * Math.sin(angleRad);

				marker.style.left = `${x}px`;
				marker.style.top = `${y}px`;
				marker.style.backgroundColor = color.hex;
				marker.style.display = "block";
			}
		});

		// Hide unused markers
		for (let i = palette.length - 1; i < this.elements.markers.length; i++) {
			if (this.elements.markers[i]) {
				this.elements.markers[i].style.display = "none";
			}
		}
	}

	/**
	 * Check accessibility contrast ratios
	 * Uses min/max for WCAG level determination
	 */
	checkAccessibility() {
		const { palette } = this.state;
		const primary = palette[0];
		const backgrounds = ["#FFFFFF", "#000000", "#1a1a2e"];

		const checks = [];
		let passCount = 0;

		backgrounds.forEach((bg) => {
			const ratio = this.getContrastRatio(primary.hex, bg);

			// WCAG levels with min contrast requirements
			const aaLarge = ratio >= 3; // Min 3:1 for large text
			const aaNormal = ratio >= 4.5; // Min 4.5:1 for normal text
			const aaaLarge = ratio >= 4.5; // Min 4.5:1 for AAA large
			const aaaNormal = ratio >= 7; // Min 7:1 for AAA normal

			if (aaNormal) passCount++;

			checks.push({
				foreground: primary.hex,
				background: bg,
				backgroundName:
					bg === "#FFFFFF" ? "White" : bg === "#000000" ? "Black" : "Dark",
				ratio: ratio.toFixed(2),
				aaLarge,
				aaNormal,
				aaaLarge,
				aaaNormal
			});
		});

		this.renderAccessibilityChecks(checks, passCount);
	}

	/**
	 * Render accessibility check results
	 */
	renderAccessibilityChecks(checks, passCount) {
		this.elements.contrastGrid.innerHTML = checks
			.map(
				(check) => `
          <div class="contrast-card">
            <div class="contrast-preview" style="background: ${
													check.background
												}; color: ${check.foreground}">
              <span class="contrast-preview-title">Sample Text</span>
              <span class="contrast-preview-text">The quick brown fox jumps over the lazy dog.</span>
            </div>
            <div class="contrast-info">
              <div class="contrast-ratio">
                <span class="ratio-value">${check.ratio}:1</span>
                <span class="ratio-label">vs ${check.backgroundName}</span>
              </div>
              <div class="wcag-levels">
                <span class="level-badge ${
																	check.aaNormal ? "pass" : ""
																}">AA</span>
                <span class="level-badge ${
																	check.aaaNormal ? "pass" : ""
																}">AAA</span>
              </div>
            </div>
          </div>
        `
			)
			.join("");

		// Update overall badge
		const overallPass = passCount >= 2;
		this.elements.wcagOverall.textContent = overallPass
			? "WCAG Pass"
			: "Needs Review";
		this.elements.wcagOverall.className = `wcag-badge ${
			overallPass ? "pass" : "fail"
		}`;
	}

	/**
	 * Calculate contrast ratio between two colors
	 * Using WCAG formula with min/max luminance ordering
	 */
	getContrastRatio(hex1, hex2) {
		const lum1 = this.getRelativeLuminance(hex1);
		const lum2 = this.getRelativeLuminance(hex2);

		// Use Math.max and Math.min to order luminances correctly
		const lighter = Math.max(lum1, lum2);
		const darker = Math.min(lum1, lum2);

		return (lighter + 0.05) / (darker + 0.05);
	}

	/**
	 * Calculate relative luminance of a color
	 */
	getRelativeLuminance(hex) {
		const rgb = this.hexToRGB(hex);
		const [r, g, b] = [rgb.r, rgb.g, rgb.b].map((v) => {
			v /= 255;
			return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
		});
		return 0.2126 * r + 0.7152 * g + 0.0722 * b;
	}

	/**
	 * Update export code based on format
	 */
	updateExportCode() {
		const { palette, exportFormat } = this.state;
		let code = "";

		switch (exportFormat) {
			case "css":
				code = this.generateCSSCode(palette);
				break;
			case "scss":
				code = this.generateSCSSCode(palette);
				break;
			case "tailwind":
				code = this.generateTailwindCode(palette);
				break;
			case "json":
				code = this.generateJSONCode(palette);
				break;
		}

		this.elements.exportCode.innerHTML = code;
	}

	/**
	 * Generate CSS custom properties code
	 */
	generateCSSCode(palette) {
		const lines = [
			'<span class="code-comment">/* Color Harmony Palette */</span>',
			'<span class="code-property">:root</span> <span class="code-bracket">{</span>'
		];

		palette.forEach((color, i) => {
			const name = i === 0 ? "primary" : `accent-${i}`;
			lines.push(
				`  <span class="code-property">--color-${name}</span>: <span class="code-string">${color.hex}</span>;`
			);
			lines.push(
				`  <span class="code-property">--color-${name}-hsl</span>: <span class="code-string">${Math.round(
					color.hue
				)}, ${color.saturation}%, ${color.lightness}%</span>;`
			);
		});

		lines.push('<span class="code-bracket">}</span>');
		return lines.join("\n");
	}

	/**
	 * Generate SCSS variables code
	 */
	generateSCSSCode(palette) {
		const lines = [
			'<span class="code-comment">// Color Harmony Palette</span>',
			""
		];

		palette.forEach((color, i) => {
			const name = i === 0 ? "primary" : `accent-${i}`;
			lines.push(
				`<span class="code-property">$color-${name}</span>: <span class="code-string">${color.hex}</span>;`
			);
		});

		lines.push("");
		lines.push('<span class="code-comment">// Color Map</span>');
		lines.push(
			'<span class="code-property">$colors</span>: <span class="code-bracket">(</span>'
		);

		palette.forEach((color, i) => {
			const name = i === 0 ? "primary" : `accent-${i}`;
			const comma = i < palette.length - 1 ? "," : "";
			lines.push(
				`  <span class="code-string">"${name}"</span>: <span class="code-string">${color.hex}</span>${comma}`
			);
		});

		lines.push('<span class="code-bracket">)</span>;');
		return lines.join("\n");
	}

	/**
	 * Generate Tailwind config code
	 */
	generateTailwindCode(palette) {
		const lines = [
			'<span class="code-comment">// tailwind.config.js</span>',
			'<span class="code-property">module.exports</span> = <span class="code-bracket">{</span>',
			'  <span class="code-property">theme</span>: <span class="code-bracket">{</span>',
			'    <span class="code-property">extend</span>: <span class="code-bracket">{</span>',
			'      <span class="code-property">colors</span>: <span class="code-bracket">{</span>'
		];

		palette.forEach((color, i) => {
			const name = i === 0 ? "primary" : `accent${i}`;
			const comma = i < palette.length - 1 ? "," : "";
			lines.push(
				`        <span class="code-property">${name}</span>: <span class="code-string">'${color.hex}'</span>${comma}`
			);
		});

		lines.push('      <span class="code-bracket">}</span>');
		lines.push('    <span class="code-bracket">}</span>');
		lines.push('  <span class="code-bracket">}</span>');
		lines.push('<span class="code-bracket">}</span>');
		return lines.join("\n");
	}

	/**
	 * Generate JSON code
	 */
	generateJSONCode(palette) {
		const data = {
			harmony: this.state.harmonyMode,
			colors: palette.map((color, i) => ({
				name: i === 0 ? "primary" : `accent-${i}`,
				hex: color.hex,
				hsl: {
					h: Math.round(color.hue),
					s: color.saturation,
					l: color.lightness
				}
			}))
		};

		const json = JSON.stringify(data, null, 2);
		return `<span class="code-string">${json.replace(
			/"/g,
			'<span class="code-value">"</span>'
		)}</span>`;
	}

	/**
	 * Randomize colors with min/max constraints
	 */
	randomize() {
		// Random hue: 0-360
		this.state.baseHue = Math.floor(Math.random() * 360);

		// Random saturation within min/max bounds
		const satRange = this.state.saturation.max - this.state.saturation.min;
		this.state.saturation.value = Math.floor(
			this.state.saturation.min + Math.random() * satRange
		);

		// Random lightness within min/max bounds
		const lightRange = this.state.lightness.max - this.state.lightness.min;
		this.state.lightness.value = Math.floor(
			this.state.lightness.min + Math.random() * lightRange
		);

		// Random harmony mode
		const modes = Object.keys(this.harmonyConfigs);
		const randomMode = modes[Math.floor(Math.random() * modes.length)];
		this.setHarmonyMode(randomMode);

		// Update UI
		this.elements.saturationRange.value = this.state.saturation.value;
		this.elements.lightnessRange.value = this.state.lightness.value;
		this.updateBaseColorPicker();
		this.generatePalette();

		this.showToast("🎲 Random palette generated!");
	}

	/**
	 * Update base color picker from HSL values
	 */
	updateBaseColorPicker() {
		const hex = this.hslToHex(
			this.state.baseHue,
			this.state.saturation.value,
			this.state.lightness.value
		);
		this.elements.baseColorPicker.value = hex;
	}

	/**
	 * Update UI elements
	 */
	updateUI() {
		const { baseHue, saturation, lightness } = this.state;
		const hex = this.hslToHex(baseHue, saturation.value, lightness.value);

		this.elements.hexValue.textContent = hex.toUpperCase();
		this.elements.hslValue.textContent = `${Math.round(baseHue)}°, ${
			saturation.value
		}%, ${lightness.value}%`;
		this.elements.saturationValue.textContent = `${saturation.value}%`;
		this.elements.lightnessValue.textContent = `${lightness.value}%`;
	}

	/**
	 * Copy color value to clipboard
	 */
	copyColorValue(type) {
		const { baseHue, saturation, lightness } = this.state;
		let value = "";

		if (type === "hex") {
			value = this.hslToHex(
				baseHue,
				saturation.value,
				lightness.value
			).toUpperCase();
		} else if (type === "hsl") {
			value = `hsl(${Math.round(baseHue)}, ${saturation.value}%, ${
				lightness.value
			}%)`;
		}

		this.copyToClipboard(value);
		this.showToast(`📋 Copied: ${value}`);
	}

	/**
	 * Copy palette card color
	 */
	copyCardColor(card) {
		const index = parseInt(card.dataset.index);
		const color = this.state.palette[index];
		if (color) {
			this.copyToClipboard(color.hex.toUpperCase());
			this.showToast(`📋 Copied: ${color.hex.toUpperCase()}`);
		}
	}

	/**
	 * Copy export code
	 */
	copyCode() {
		const code = this.elements.exportCode.textContent;
		this.copyToClipboard(code);
		this.showToast("📋 Code copied to clipboard!");
	}

	/**
	 * Download palette as file
	 */
	downloadPalette() {
		const { palette, exportFormat, harmonyMode } = this.state;
		let content = "";
		let filename = `palette-${harmonyMode}`;
		let mimeType = "text/plain";

		switch (exportFormat) {
			case "css":
				content = `:root {\n${palette
					.map(
						(c, i) => `  --color-${i === 0 ? "primary" : `accent-${i}`}: ${c.hex};`
					)
					.join("\n")}\n}`;
				filename += ".css";
				mimeType = "text/css";
				break;
			case "scss":
				content = palette
					.map((c, i) => `$color-${i === 0 ? "primary" : `accent-${i}`}: ${c.hex};`)
					.join("\n");
				filename += ".scss";
				break;
			case "json":
				content = JSON.stringify(
					{
						harmony: harmonyMode,
						colors: palette.map((c, i) => ({
							name: i === 0 ? "primary" : `accent-${i}`,
							hex: c.hex,
							hsl: { h: Math.round(c.hue), s: c.saturation, l: c.lightness }
						}))
					},
					null,
					2
				);
				filename += ".json";
				mimeType = "application/json";
				break;
			default:
				content = palette.map((c) => c.hex).join("\n");
				filename += ".txt";
		}

		const blob = new Blob([content], { type: mimeType });
		const url = URL.createObjectURL(blob);
		const a = document.createElement("a");
		a.href = url;
		a.download = filename;
		a.click();
		URL.revokeObjectURL(url);

		this.showToast(`📥 Downloaded ${filename}`);
	}

	/**
	 * Copy text to clipboard
	 */
	async copyToClipboard(text) {
		try {
			await navigator.clipboard.writeText(text);
		} catch (err) {
			// Fallback for older browsers
			const textarea = document.createElement("textarea");
			textarea.value = text;
			document.body.appendChild(textarea);
			textarea.select();
			document.execCommand("copy");
			document.body.removeChild(textarea);
		}
	}

	/**
	 * Show toast notification
	 */
	showToast(message) {
		this.elements.toast.textContent = message;
		this.elements.toast.classList.add("show", "success");

		setTimeout(() => {
			this.elements.toast.classList.remove("show", "success");
		}, 2500);
	}

	// ============================================
	// COLOR CONVERSION UTILITIES
	// ============================================

	/**
	 * Convert HEX to HSL
	 */
	hexToHSL(hex) {
		const rgb = this.hexToRGB(hex);
		const r = rgb.r / 255;
		const g = rgb.g / 255;
		const b = rgb.b / 255;

		// Use Math.min and Math.max for finding luminance range
		const max = Math.max(r, g, b);
		const min = Math.min(r, g, b);
		const l = (max + min) / 2;

		let h = 0,
			s = 0;

		if (max !== min) {
			const d = max - min;
			s = l > 0.5 ? d / (2 - max - min) : d / (max + min);

			switch (max) {
				case r:
					h = ((g - b) / d + (g < b ? 6 : 0)) / 6;
					break;
				case g:
					h = ((b - r) / d + 2) / 6;
					break;
				case b:
					h = ((r - g) / d + 4) / 6;
					break;
			}
		}

		return {
			h: Math.round(h * 360),
			s: Math.round(s * 100),
			l: Math.round(l * 100)
		};
	}

	/**
	 * Convert HSL to HEX
	 */
	hslToHex(h, s, l) {
		s /= 100;
		l /= 100;

		const c = (1 - Math.abs(2 * l - 1)) * s;
		const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
		const m = l - c / 2;

		let r = 0,
			g = 0,
			b = 0;

		if (h >= 0 && h < 60) {
			r = c;
			g = x;
			b = 0;
		} else if (h >= 60 && h < 120) {
			r = x;
			g = c;
			b = 0;
		} else if (h >= 120 && h < 180) {
			r = 0;
			g = c;
			b = x;
		} else if (h >= 180 && h < 240) {
			r = 0;
			g = x;
			b = c;
		} else if (h >= 240 && h < 300) {
			r = x;
			g = 0;
			b = c;
		} else {
			r = c;
			g = 0;
			b = x;
		}

		// Clamp values using Math.min and Math.max
		r = Math.round(Math.min(255, Math.max(0, (r + m) * 255)));
		g = Math.round(Math.min(255, Math.max(0, (g + m) * 255)));
		b = Math.round(Math.min(255, Math.max(0, (b + m) * 255)));

		return `#${r.toString(16).padStart(2, "0")}${g
			.toString(16)
			.padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
	}

	/**
	 * Convert HEX to RGB
	 */
	hexToRGB(hex) {
		const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
		return result
			? {
					r: parseInt(result[1], 16),
					g: parseInt(result[2], 16),
					b: parseInt(result[3], 16)
			  }
			: { r: 0, g: 0, b: 0 };
	}

	/**
	 * Get contrasting text color (black or white)
	 */
	getContrastColor(hex) {
		const lum = this.getRelativeLuminance(hex);
		return lum > 0.179 ? "#000000" : "#FFFFFF";
	}

	/**
	 * Get approximate color name based on hue
	 */
	getColorName(hue) {
		// Find closest named hue using Math.min distance
		const hues = Object.keys(this.colorNames).map(Number);
		let closestHue = hues[0];
		let minDistance = 360;

		hues.forEach((h) => {
			let distance = Math.abs(hue - h);
			// Handle circular hue (0 and 360 are the same)
			distance = Math.min(distance, 360 - distance);

			if (distance < minDistance) {
				minDistance = distance;
				closestHue = h;
			}
		});

		return this.colorNames[closestHue];
	}

	/**
	 * Format harmony mode name for display
	 */
	formatModeName(mode) {
		return mode.charAt(0).toUpperCase() + mode.slice(1).replace("-", " ");
	}
}

// Initialize the app when DOM is ready
document.addEventListener("DOMContentLoaded", () => {
	new ColorHarmonyGenerator();
});