/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./data/client/ts/bg.ts":
/*!******************************!*\
  !*** ./data/client/ts/bg.ts ***!
  \******************************/
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
const particle_1 = __importDefault(__webpack_require__(/*! ./bgResources/particle */ "./data/client/ts/bgResources/particle.ts"));
const canvas = document.createElement("canvas");
document.body.appendChild(canvas);
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
canvas.classList.add("bg");
const ctx = canvas.getContext("2d");
let width = canvas.width;
let height = canvas.height;
let lastFrame = Date.now();
let particles = [];
let mouse = {
    x: 0,
    y: 0,
};
for (let i = 0; i < 100; i++) {
    let p = new particle_1.default({
        x: Math.random() * width,
        y: Math.random() * height,
    });
    p.setVelocity({
        x: (Math.random() * 2 - 1) / 50,
        y: (Math.random() * 2 - 1) / 50,
    });
    particles.push(p);
}
document.addEventListener("resize", (ev) => {
    width = canvas.width;
    height = canvas.height;
});
document.addEventListener("mousemove", (ev) => {
    mouse.x = ev.clientX;
    mouse.y = ev.clientY;
});
function renderFrame() {
    const now = Date.now();
    const delta = now - lastFrame;
    lastFrame = now;
    ctx.clearRect(0, 0, width, height);
    const backgroundGradient = ctx.createLinearGradient(0, 0, 0, width);
    backgroundGradient.addColorStop(0, "#000000");
    backgroundGradient.addColorStop(0.33, "#000000");
    backgroundGradient.addColorStop(1, "#550055");
    ctx.fillStyle = backgroundGradient;
    ctx.fillRect(0, 0, width, height);
    for (let i = 0; i < particles.length; i++) {
        const p = particles[i];
        p.move(delta, "wrap", { x: 0, y: 0, width, height });
        p.draw(ctx, mouse);
    }
}
setInterval(renderFrame, 1000 / 60);


/***/ }),

/***/ "./data/client/ts/bgResources/particle.ts":
/*!************************************************!*\
  !*** ./data/client/ts/bgResources/particle.ts ***!
  \************************************************/
/***/ ((__unused_webpack_module, exports) => {


Object.defineProperty(exports, "__esModule", ({ value: true }));
class Particle {
    constructor(position) {
        this.velocity = {
            x: 0,
            y: 0,
        };
        this.position = position;
    }
    setVelocity(v) {
        this.velocity = v;
    }
    move(delta, mode = "wrap", bounds) {
        this.position.x += this.velocity.x * delta;
        this.position.y += this.velocity.y * delta;
        if (!bounds)
            bounds = { x: 0, y: 0, width: window.innerWidth, height: window.innerHeight };
        if (mode === "bounce") {
            if (this.position.x < bounds.x) {
                this.position.x = bounds.x;
                this.velocity.x *= -1;
            }
            else if (this.position.x > bounds.x + bounds.width) {
                this.position.x = bounds.x + bounds.width;
                this.velocity.x *= -1;
            }
            if (this.position.y < bounds.y) {
                this.position.y = bounds.y;
                this.velocity.y *= -1;
            }
            else if (this.position.y > bounds.y + bounds.height) {
                this.position.y = bounds.y + bounds.height;
                this.velocity.y *= -1;
            }
        }
        if (mode === "wrap") {
            if (this.position.x < bounds.x) {
                this.position.x = bounds.x + bounds.width;
            }
            else if (this.position.x > bounds.x + bounds.width) {
                this.position.x = bounds.x;
            }
            if (this.position.y < bounds.y) {
                this.position.y = bounds.y + bounds.height;
            }
            else if (this.position.y > bounds.y + bounds.height) {
                this.position.y = bounds.y;
            }
        }
    }
    draw(ctx, mouse) {
        const distance = Math.sqrt(Math.pow(this.position.x - mouse.x, 2) + Math.pow(this.position.y - mouse.y, 2));
        const size = Math.max(10, 100 - distance) / 10;
        ctx.fillStyle = "#fff";
        ctx.beginPath();
        ctx.arc(this.position.x, this.position.y, size, 0, Math.PI * 2);
        ctx.fill();
    }
}
exports["default"] = Particle;


/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module is referenced by other modules so it can't be inlined
/******/ 	var __webpack_exports__ = __webpack_require__("./data/client/ts/bg.ts");
/******/ 	
/******/ })()
;
//# sourceMappingURL=data:application/json;charset=utf-8;base64,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USxDQUFDLENBQUMsR0FBRyxLQUFLLENBQUMsQ0FBQyxFQUFFLENBQUMsQ0FBQyxDQUNoRixDQUFDO1FBRUYsTUFBTSxJQUFJLEdBQUcsSUFBSSxDQUFDLEdBQUcsQ0FBQyxFQUFFLEVBQUUsR0FBRyxHQUFHLFFBQVEsQ0FBQyxHQUFHLEVBQUUsQ0FBQztRQUUvQyxHQUFHLENBQUMsU0FBUyxHQUFHLE1BQU0sQ0FBQztRQUV2QixHQUFHLENBQUMsU0FBUyxFQUFFLENBQUM7UUFDaEIsR0FBRyxDQUFDLEdBQUcsQ0FBQyxJQUFJLENBQUMsUUFBUSxDQUFDLENBQUMsRUFBRSxJQUFJLENBQUMsUUFBUSxDQUFDLENBQUMsRUFBRSxJQUFJLEVBQUUsQ0FBQyxFQUFFLElBQUksQ0FBQyxFQUFFLEdBQUcsQ0FBQyxDQUFDLENBQUM7UUFDaEUsR0FBRyxDQUFDLElBQUksRUFBRSxDQUFDO0lBRWIsQ0FBQztDQUNGO0FBN0VELDhCQTZFQzs7Ozs7OztVQzdFRDtVQUNBOztVQUVBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBOztVQUVBO1VBQ0E7O1VBRUE7VUFDQTtVQUNBOzs7O1VFdEJBO1VBQ0E7VUFDQTtVQUNBIiwic291cmNlcyI6WyJ3ZWJwYWNrOi8vYXhvbG90bGNsaWVudC1hcGkvLi9kYXRhL2NsaWVudC90cy9iZy50cyIsIndlYnBhY2s6Ly9heG9sb3RsY2xpZW50LWFwaS8uL2RhdGEvY2xpZW50L3RzL2JnUmVzb3VyY2VzL3BhcnRpY2xlLnRzIiwid2VicGFjazovL2F4b2xvdGxjbGllbnQtYXBpL3dlYnBhY2svYm9vdHN0cmFwIiwid2VicGFjazovL2F4b2xvdGxjbGllbnQtYXBpL3dlYnBhY2svYmVmb3JlLXN0YXJ0dXAiLCJ3ZWJwYWNrOi8vYXhvbG90bGNsaWVudC1hcGkvd2VicGFjay9zdGFydHVwIiwid2VicGFjazovL2F4b2xvdGxjbGllbnQtYXBpL3dlYnBhY2svYWZ0ZXItc3RhcnR1cCJdLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgUGFydGljbGUgZnJvbSBcIi4vYmdSZXNvdXJjZXMvcGFydGljbGVcIjtcblxuY29uc3QgY2FudmFzID0gZG9jdW1lbnQuY3JlYXRlRWxlbWVudChcImNhbnZhc1wiKTtcbmRvY3VtZW50LmJvZHkuYXBwZW5kQ2hpbGQoY2FudmFzKTtcblxuY2FudmFzLndpZHRoID0gd2luZG93LmlubmVyV2lkdGg7XG5jYW52YXMuaGVpZ2h0ID0gd2luZG93LmlubmVySGVpZ2h0O1xuXG5jYW52YXMuY2xhc3NMaXN0LmFkZChcImJnXCIpO1xuXG5jb25zdCBjdHggPSBjYW52YXMuZ2V0Q29udGV4dChcIjJkXCIpIGFzIENhbnZhc1JlbmRlcmluZ0NvbnRleHQyRDtcblxubGV0IHdpZHRoID0gY2FudmFzLndpZHRoO1xubGV0IGhlaWdodCA9IGNhbnZhcy5oZWlnaHQ7XG5cbmxldCBsYXN0RnJhbWUgPSBEYXRlLm5vdygpO1xuXG5sZXQgcGFydGljbGVzOiBQYXJ0aWNsZVtdID0gW107XG5cbmxldCBtb3VzZSA9IHtcbiAgICB4OiAwLFxuICAgIHk6IDAsXG59O1xuXG5mb3IgKGxldCBpID0gMDsgaSA8IDEwMDsgaSsrKSB7XG4gIGxldCBwID0gbmV3IFBhcnRpY2xlKHtcbiAgICB4OiBNYXRoLnJhbmRvbSgpICogd2lkdGgsXG4gICAgeTogTWF0aC5yYW5kb20oKSAqIGhlaWdodCxcbiAgfSk7XG5cbiAgcC5zZXRWZWxvY2l0eSh7XG4gICAgeDogKE1hdGgucmFuZG9tKCkgKiAyIC0gMSkgLyA1MCxcbiAgICB5OiAoTWF0aC5yYW5kb20oKSAqIDIgLSAxKSAvIDUwLFxuICB9KTtcblxuICBwYXJ0aWNsZXMucHVzaChwKTtcbn1cblxuZG9jdW1lbnQuYWRkRXZlbnRMaXN0ZW5lcihcInJlc2l6ZVwiLCAoZXYpID0+IHtcbiAgd2lkdGggPSBjYW52YXMud2lkdGg7XG4gIGhlaWdodCA9IGNhbnZhcy5oZWlnaHQ7XG59KTtcblxuZG9jdW1lbnQuYWRkRXZlbnRMaXN0ZW5lcihcIm1vdXNlbW92ZVwiLCAoZXYpID0+IHtcbiAgICBtb3VzZS54ID0gZXYuY2xpZW50WDtcbiAgICBtb3VzZS55ID0gZXYuY2xpZW50WTtcbn0pO1xuXG5mdW5jdGlvbiByZW5kZXJGcmFtZSgpIHtcbiAgY29uc3Qgbm93ID0gRGF0ZS5ub3coKTtcbiAgY29uc3QgZGVsdGEgPSBub3cgLSBsYXN0RnJhbWU7XG4gIGxhc3RGcmFtZSA9IG5vdztcblxuICBjdHguY2xlYXJSZWN0KDAsIDAsIHdpZHRoLCBoZWlnaHQpO1xuXG4gIGNvbnN0IGJhY2tncm91bmRHcmFkaWVudCA9IGN0eC5jcmVhdGVMaW5lYXJHcmFkaWVudCgwLCAwLCAwLCB3aWR0aCk7XG5cbiAgYmFja2dyb3VuZEdyYWRpZW50LmFkZENvbG9yU3RvcCgwLCBcIiMwMDAwMDBcIik7XG4gIGJhY2tncm91bmRHcmFkaWVudC5hZGRDb2xvclN0b3AoMC4zMywgXCIjMDAwMDAwXCIpO1xuICBiYWNrZ3JvdW5kR3JhZGllbnQuYWRkQ29sb3JTdG9wKDEsIFwiIzU1MDA1NVwiKTtcblxuICBjdHguZmlsbFN0eWxlID0gYmFja2dyb3VuZEdyYWRpZW50O1xuXG4gIGN0eC5maWxsUmVjdCgwLCAwLCB3aWR0aCwgaGVpZ2h0KTtcblxuICBmb3IgKGxldCBpID0gMDsgaSA8IHBhcnRpY2xlcy5sZW5ndGg7IGkrKykge1xuICAgIGNvbnN0IHAgPSBwYXJ0aWNsZXNbaV07XG5cbiAgICBwLm1vdmUoZGVsdGEsIFwid3JhcFwiLCB7IHg6IDAsIHk6IDAsIHdpZHRoLCBoZWlnaHQgfSk7XG4gICAgcC5kcmF3KGN0eCwgbW91c2UpO1xuICB9XG59XG5cbnNldEludGVydmFsKHJlbmRlckZyYW1lLCAxMDAwIC8gNjApO1xuIiwiZXhwb3J0IGRlZmF1bHQgY2xhc3MgUGFydGljbGUge1xuICBwdWJsaWMgcG9zaXRpb246IFBvc2l0aW9uO1xuICBwdWJsaWMgdmVsb2NpdHk6IFBvc2l0aW9uID0ge1xuICAgIHg6IDAsXG4gICAgeTogMCxcbiAgfTtcblxuICBjb25zdHJ1Y3Rvcihwb3NpdGlvbjogUG9zaXRpb24pIHtcbiAgICB0aGlzLnBvc2l0aW9uID0gcG9zaXRpb247XG4gIH1cblxuICBwdWJsaWMgc2V0VmVsb2NpdHkodjogUG9zaXRpb24pIHtcbiAgICB0aGlzLnZlbG9jaXR5ID0gdjtcbiAgfVxuXG4gIHB1YmxpYyBtb3ZlKFxuICAgIGRlbHRhOiBudW1iZXIsXG4gICAgbW9kZTogXCJib3VuY2VcIiB8IFwid3JhcFwiID0gXCJ3cmFwXCIsXG4gICAgYm91bmRzPzoge1xuICAgICAgeDogbnVtYmVyO1xuICAgICAgeTogbnVtYmVyO1xuICAgICAgd2lkdGg6IG51bWJlcjtcbiAgICAgIGhlaWdodDogbnVtYmVyO1xuICAgIH1cbiAgKSB7XG4gICAgdGhpcy5wb3NpdGlvbi54ICs9IHRoaXMudmVsb2NpdHkueCAqIGRlbHRhO1xuICAgIHRoaXMucG9zaXRpb24ueSArPSB0aGlzLnZlbG9jaXR5LnkgKiBkZWx0YTtcblxuICAgIGlmICghYm91bmRzKSBib3VuZHMgPSB7IHg6IDAsIHk6IDAsIHdpZHRoOiB3aW5kb3cuaW5uZXJXaWR0aCwgaGVpZ2h0OiB3aW5kb3cuaW5uZXJIZWlnaHQgfTtcblxuICAgIGlmIChtb2RlID09PSBcImJvdW5jZVwiKSB7XG4gICAgICBpZiAodGhpcy5wb3NpdGlvbi54IDwgYm91bmRzLngpIHtcbiAgICAgICAgdGhpcy5wb3NpdGlvbi54ID0gYm91bmRzLng7XG4gICAgICAgIHRoaXMudmVsb2NpdHkueCAqPSAtMTtcbiAgICAgIH0gZWxzZSBpZiAodGhpcy5wb3NpdGlvbi54ID4gYm91bmRzLnggKyBib3VuZHMud2lkdGgpIHtcbiAgICAgICAgdGhpcy5wb3NpdGlvbi54ID0gYm91bmRzLnggKyBib3VuZHMud2lkdGg7XG4gICAgICAgIHRoaXMudmVsb2NpdHkueCAqPSAtMTtcbiAgICAgIH1cblxuICAgICAgaWYgKHRoaXMucG9zaXRpb24ueSA8IGJvdW5kcy55KSB7XG4gICAgICAgIHRoaXMucG9zaXRpb24ueSA9IGJvdW5kcy55O1xuICAgICAgICB0aGlzLnZlbG9jaXR5LnkgKj0gLTE7XG4gICAgICB9IGVsc2UgaWYgKHRoaXMucG9zaXRpb24ueSA+IGJvdW5kcy55ICsgYm91bmRzLmhlaWdodCkge1xuICAgICAgICB0aGlzLnBvc2l0aW9uLnkgPSBib3VuZHMueSArIGJvdW5kcy5oZWlnaHQ7XG4gICAgICAgIHRoaXMudmVsb2NpdHkueSAqPSAtMTtcbiAgICAgIH1cbiAgICB9XG5cbiAgICBpZiAobW9kZSA9PT0gXCJ3cmFwXCIpIHtcbiAgICAgIGlmICh0aGlzLnBvc2l0aW9uLnggPCBib3VuZHMueCkge1xuICAgICAgICB0aGlzLnBvc2l0aW9uLnggPSBib3VuZHMueCArIGJvdW5kcy53aWR0aDtcbiAgICAgIH0gZWxzZSBpZiAodGhpcy5wb3NpdGlvbi54ID4gYm91bmRzLnggKyBib3VuZHMud2lkdGgpIHtcbiAgICAgICAgdGhpcy5wb3NpdGlvbi54ID0gYm91bmRzLng7XG4gICAgICB9XG5cbiAgICAgIGlmICh0aGlzLnBvc2l0aW9uLnkgPCBib3VuZHMueSkge1xuICAgICAgICB0aGlzLnBvc2l0aW9uLnkgPSBib3VuZHMueSArIGJvdW5kcy5oZWlnaHQ7XG4gICAgICB9IGVsc2UgaWYgKHRoaXMucG9zaXRpb24ueSA+IGJvdW5kcy55ICsgYm91bmRzLmhlaWdodCkge1xuICAgICAgICB0aGlzLnBvc2l0aW9uLnkgPSBib3VuZHMueTtcbiAgICAgIH1cbiAgICB9XG4gIH1cblxuICBwdWJsaWMgZHJhdyhjdHg6IENhbnZhc1JlbmRlcmluZ0NvbnRleHQyRCwgbW91c2U6IFBvc2l0aW9uKSB7XG4gICAgY29uc3QgZGlzdGFuY2UgPSBNYXRoLnNxcnQoXG4gICAgICBNYXRoLnBvdyh0aGlzLnBvc2l0aW9uLnggLSBtb3VzZS54LCAyKSArIE1hdGgucG93KHRoaXMucG9zaXRpb24ueSAtIG1vdXNlLnksIDIpXG4gICAgKTtcblxuICAgIGNvbnN0IHNpemUgPSBNYXRoLm1heCgxMCwgMTAwIC0gZGlzdGFuY2UpIC8gMTA7XG5cbiAgICBjdHguZmlsbFN0eWxlID0gXCIjZmZmXCI7XG4gICAgXG4gICAgY3R4LmJlZ2luUGF0aCgpO1xuICAgIGN0eC5hcmModGhpcy5wb3NpdGlvbi54LCB0aGlzLnBvc2l0aW9uLnksIHNpemUsIDAsIE1hdGguUEkgKiAyKTtcbiAgICBjdHguZmlsbCgpO1xuICAgIFxuICB9XG59XG5cbiIsIi8vIFRoZSBtb2R1bGUgY2FjaGVcbnZhciBfX3dlYnBhY2tfbW9kdWxlX2NhY2hlX18gPSB7fTtcblxuLy8gVGhlIHJlcXVpcmUgZnVuY3Rpb25cbmZ1bmN0aW9uIF9fd2VicGFja19yZXF1aXJlX18obW9kdWxlSWQpIHtcblx0Ly8gQ2hlY2sgaWYgbW9kdWxlIGlzIGluIGNhY2hlXG5cdHZhciBjYWNoZWRNb2R1bGUgPSBfX3dlYnBhY2tfbW9kdWxlX2NhY2hlX19bbW9kdWxlSWRdO1xuXHRpZiAoY2FjaGVkTW9kdWxlICE9PSB1bmRlZmluZWQpIHtcblx0XHRyZXR1cm4gY2FjaGVkTW9kdWxlLmV4cG9ydHM7XG5cdH1cblx0Ly8gQ3JlYXRlIGEgbmV3IG1vZHVsZSAoYW5kIHB1dCBpdCBpbnRvIHRoZSBjYWNoZSlcblx0dmFyIG1vZHVsZSA9IF9fd2VicGFja19tb2R1bGVfY2FjaGVfX1ttb2R1bGVJZF0gPSB7XG5cdFx0Ly8gbm8gbW9kdWxlLmlkIG5lZWRlZFxuXHRcdC8vIG5vIG1vZHVsZS5sb2FkZWQgbmVlZGVkXG5cdFx0ZXhwb3J0czoge31cblx0fTtcblxuXHQvLyBFeGVjdXRlIHRoZSBtb2R1bGUgZnVuY3Rpb25cblx0X193ZWJwYWNrX21vZHVsZXNfX1ttb2R1bGVJZF0uY2FsbChtb2R1bGUuZXhwb3J0cywgbW9kdWxlLCBtb2R1bGUuZXhwb3J0cywgX193ZWJwYWNrX3JlcXVpcmVfXyk7XG5cblx0Ly8gUmV0dXJuIHRoZSBleHBvcnRzIG9mIHRoZSBtb2R1bGVcblx0cmV0dXJuIG1vZHVsZS5leHBvcnRzO1xufVxuXG4iLCIiLCIvLyBzdGFydHVwXG4vLyBMb2FkIGVudHJ5IG1vZHVsZSBhbmQgcmV0dXJuIGV4cG9ydHNcbi8vIFRoaXMgZW50cnkgbW9kdWxlIGlzIHJlZmVyZW5jZWQgYnkgb3RoZXIgbW9kdWxlcyBzbyBpdCBjYW4ndCBiZSBpbmxpbmVkXG52YXIgX193ZWJwYWNrX2V4cG9ydHNfXyA9IF9fd2VicGFja19yZXF1aXJlX18oXCIuL2RhdGEvY2xpZW50L3RzL2JnLnRzXCIpO1xuIiwiIl0sIm5hbWVzIjpbXSwic291cmNlUm9vdCI6IiJ9