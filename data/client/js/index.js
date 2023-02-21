/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./data/client/ts/index.ts":
/*!*********************************!*\
  !*** ./data/client/ts/index.ts ***!
  \*********************************/
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
const fadeIn_1 = __importDefault(__webpack_require__(/*! ./indexResources/fadeIn */ "./data/client/ts/indexResources/fadeIn.ts"));
fadeIn_1.default.runFadeIn({
    initalDelay: 500,
    spacing: 100,
});
// get user platform (windows, mac, linux) for download links
// @ts-ignore
const platformID = navigator.platform.toLowerCase();
let platform;
if (platformID.includes("win")) {
    platform = "windows";
}
else if (platformID.includes("mac")) {
    if (platformID.includes("arm")) {
        platform = "mac-arm64";
    }
    else {
        platform = "mac-x64";
    }
}
else if (platformID.includes("linux")) {
    platform = "linux";
}
else {
    platform = "windows";
}
let platformExtension;
if (platform === "windows") {
    platformExtension = "exe";
}
else if (platform === "linux") {
    platformExtension = "AppImage";
}
else if (platform === "mac-x64") {
    platformExtension = "-x64.dmg";
}
else if (platform === "mac-arm64") {
    platformExtension = "-arm64.dmg";
}
let formattedPlatform;
if (platform === "windows") {
    formattedPlatform = "Windows";
}
else if (platform === "linux") {
    formattedPlatform = "Linux";
}
else if (platform === "mac-x64") {
    formattedPlatform = "Mac (Intel)";
}
else if (platform === "mac-arm64") {
    formattedPlatform = "Mac (Apple Silicon)";
}
// set download link
// get status data
(() => __awaiter(void 0, void 0, void 0, function* () {
    // fetch client data
    const clientData = (yield fetch("/api/v1/count").then((res) => res.json()));
    // set client count
    document.getElementById("users").innerHTML = clientData.total.toLocaleString();
    document.getElementById("online").innerHTML = clientData.online.toLocaleString();
    // fetch modrinth data
    const modrinthData = (yield fetch("https://api.modrinth.com/v2/project/axolotlclient").then((res) => res.json()));
    // set modrinth download count
    document.getElementById("downloads").innerHTML = modrinthData.downloads.toLocaleString();
    // fetch discord data
    const discordData = (yield fetch("https://discord.com/api/guilds/872856682567454720/widget.json").then((res) => res.json()));
    // set discord member count
    document.getElementById("discord").innerHTML = discordData.presence_count.toLocaleString();
}))();


/***/ }),

/***/ "./data/client/ts/indexResources/fadeIn.ts":
/*!*************************************************!*\
  !*** ./data/client/ts/indexResources/fadeIn.ts ***!
  \*************************************************/
/***/ ((__unused_webpack_module, exports) => {


Object.defineProperty(exports, "__esModule", ({ value: true }));
class FadeIn {
    static runFadeIn(options) {
        // set default options
        const elements = document.querySelectorAll("#fade-in");
        // order by `data-fade-order` attribute
        const orderedElements = Array.from(elements).map((element) => {
            return {
                element,
                order: parseInt(element.getAttribute("data-fade-order") || "0"),
            };
        });
        // fade in
        orderedElements.forEach((item) => {
            item.element.classList.add("hidden");
            setTimeout(() => {
                item.element.classList.add("fade-in");
                item.element.classList.remove("hidden");
            }, item.order * options.spacing + options.initalDelay);
        });
    }
}
exports["default"] = FadeIn;


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
/******/ 	var __webpack_exports__ = __webpack_require__("./data/client/ts/index.ts");
/******/ 	
/******/ })()
;
//# sourceMappingURL=data:application/json;charset=utf-8;base64,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T0FBTyxDQUFDLE9BQU8sR0FBRyxPQUFPLENBQUMsV0FBVyxDQUFDLENBQUM7UUFDM0QsQ0FBQyxDQUFDO0lBRU4sQ0FBQztDQUVKO0FBakNELDRCQWlDQzs7Ozs7OztVQ2pDRDtVQUNBOztVQUVBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBOztVQUVBO1VBQ0E7O1VBRUE7VUFDQTtVQUNBOzs7O1VFdEJBO1VBQ0E7VUFDQTtVQUNBIiwic291cmNlcyI6WyJ3ZWJwYWNrOi8vYXhvbG90bGNsaWVudC1hcGkvLi9kYXRhL2NsaWVudC90cy9pbmRleC50cyIsIndlYnBhY2s6Ly9heG9sb3RsY2xpZW50LWFwaS8uL2RhdGEvY2xpZW50L3RzL2luZGV4UmVzb3VyY2VzL2ZhZGVJbi50cyIsIndlYnBhY2s6Ly9heG9sb3RsY2xpZW50LWFwaS93ZWJwYWNrL2Jvb3RzdHJhcCIsIndlYnBhY2s6Ly9heG9sb3RsY2xpZW50LWFwaS93ZWJwYWNrL2JlZm9yZS1zdGFydHVwIiwid2VicGFjazovL2F4b2xvdGxjbGllbnQtYXBpL3dlYnBhY2svc3RhcnR1cCIsIndlYnBhY2s6Ly9heG9sb3RsY2xpZW50LWFwaS93ZWJwYWNrL2FmdGVyLXN0YXJ0dXAiXSwic291cmNlc0NvbnRlbnQiOlsiaW1wb3J0IEZhZGVJbiBmcm9tIFwiLi9pbmRleFJlc291cmNlcy9mYWRlSW5cIjtcblxuRmFkZUluLnJ1bkZhZGVJbih7XG4gIGluaXRhbERlbGF5OiA1MDAsXG4gIHNwYWNpbmc6IDEwMCxcbn0pO1xuXG4vLyBnZXQgdXNlciBwbGF0Zm9ybSAod2luZG93cywgbWFjLCBsaW51eCkgZm9yIGRvd25sb2FkIGxpbmtzXG4vLyBAdHMtaWdub3JlXG5jb25zdCBwbGF0Zm9ybUlEOiBzdHJpbmcgPSBuYXZpZ2F0b3IucGxhdGZvcm0udG9Mb3dlckNhc2UoKTtcblxubGV0IHBsYXRmb3JtOiBcIndpbmRvd3NcIiB8IFwibGludXhcIiB8IFwibWFjLXg2NFwiIHwgXCJtYWMtYXJtNjRcIjtcblxuaWYgKHBsYXRmb3JtSUQuaW5jbHVkZXMoXCJ3aW5cIikpIHtcbiAgcGxhdGZvcm0gPSBcIndpbmRvd3NcIjtcbn0gZWxzZSBpZiAocGxhdGZvcm1JRC5pbmNsdWRlcyhcIm1hY1wiKSkge1xuICBpZiAocGxhdGZvcm1JRC5pbmNsdWRlcyhcImFybVwiKSkge1xuICAgIHBsYXRmb3JtID0gXCJtYWMtYXJtNjRcIjtcbiAgfSBlbHNlIHtcbiAgICBwbGF0Zm9ybSA9IFwibWFjLXg2NFwiO1xuICB9XG59IGVsc2UgaWYgKHBsYXRmb3JtSUQuaW5jbHVkZXMoXCJsaW51eFwiKSkge1xuICBwbGF0Zm9ybSA9IFwibGludXhcIjtcbn0gZWxzZSB7XG4gIHBsYXRmb3JtID0gXCJ3aW5kb3dzXCI7XG59XG5cbmxldCBwbGF0Zm9ybUV4dGVuc2lvbjogc3RyaW5nO1xuXG5pZiAocGxhdGZvcm0gPT09IFwid2luZG93c1wiKSB7XG4gIHBsYXRmb3JtRXh0ZW5zaW9uID0gXCJleGVcIjtcbn0gZWxzZSBpZiAocGxhdGZvcm0gPT09IFwibGludXhcIikge1xuICBwbGF0Zm9ybUV4dGVuc2lvbiA9IFwiQXBwSW1hZ2VcIjtcbn0gZWxzZSBpZiAocGxhdGZvcm0gPT09IFwibWFjLXg2NFwiKSB7XG4gIHBsYXRmb3JtRXh0ZW5zaW9uID0gXCIteDY0LmRtZ1wiO1xufSBlbHNlIGlmIChwbGF0Zm9ybSA9PT0gXCJtYWMtYXJtNjRcIikge1xuICBwbGF0Zm9ybUV4dGVuc2lvbiA9IFwiLWFybTY0LmRtZ1wiO1xufVxuXG5sZXQgZm9ybWF0dGVkUGxhdGZvcm06IHN0cmluZztcblxuaWYgKHBsYXRmb3JtID09PSBcIndpbmRvd3NcIikge1xuICBmb3JtYXR0ZWRQbGF0Zm9ybSA9IFwiV2luZG93c1wiO1xufSBlbHNlIGlmIChwbGF0Zm9ybSA9PT0gXCJsaW51eFwiKSB7XG4gIGZvcm1hdHRlZFBsYXRmb3JtID0gXCJMaW51eFwiO1xufSBlbHNlIGlmIChwbGF0Zm9ybSA9PT0gXCJtYWMteDY0XCIpIHtcbiAgZm9ybWF0dGVkUGxhdGZvcm0gPSBcIk1hYyAoSW50ZWwpXCI7XG59IGVsc2UgaWYgKHBsYXRmb3JtID09PSBcIm1hYy1hcm02NFwiKSB7XG4gIGZvcm1hdHRlZFBsYXRmb3JtID0gXCJNYWMgKEFwcGxlIFNpbGljb24pXCI7XG59XG5cbi8vIHNldCBkb3dubG9hZCBsaW5rXG5cbi8vIGdldCBzdGF0dXMgZGF0YVxuXG4oYXN5bmMgKCkgPT4ge1xuICAvLyBmZXRjaCBjbGllbnQgZGF0YVxuICBjb25zdCBjbGllbnREYXRhID0gKGF3YWl0IGZldGNoKFwiL2FwaS92MS9jb3VudFwiKS50aGVuKChyZXMpID0+IHJlcy5qc29uKCkpKSBhcyB7XG4gICAgb25saW5lOiBudW1iZXI7XG4gICAgdG90YWw6IG51bWJlcjtcbiAgfTtcblxuICAvLyBzZXQgY2xpZW50IGNvdW50XG4gIGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwidXNlcnNcIikhLmlubmVySFRNTCA9IGNsaWVudERhdGEudG90YWwudG9Mb2NhbGVTdHJpbmcoKTtcbiAgZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJvbmxpbmVcIikhLmlubmVySFRNTCA9IGNsaWVudERhdGEub25saW5lLnRvTG9jYWxlU3RyaW5nKCk7XG5cbiAgLy8gZmV0Y2ggbW9kcmludGggZGF0YVxuICBjb25zdCBtb2RyaW50aERhdGEgPSAoYXdhaXQgZmV0Y2goXCJodHRwczovL2FwaS5tb2RyaW50aC5jb20vdjIvcHJvamVjdC9heG9sb3RsY2xpZW50XCIpLnRoZW4oKHJlcykgPT5cbiAgICByZXMuanNvbigpXG4gICkpIGFzIHtcbiAgICBkb3dubG9hZHM6IG51bWJlcjtcbiAgfTtcblxuICAvLyBzZXQgbW9kcmludGggZG93bmxvYWQgY291bnRcbiAgZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJkb3dubG9hZHNcIikhLmlubmVySFRNTCA9IG1vZHJpbnRoRGF0YS5kb3dubG9hZHMudG9Mb2NhbGVTdHJpbmcoKTtcblxuICAvLyBmZXRjaCBkaXNjb3JkIGRhdGFcblxuICBjb25zdCBkaXNjb3JkRGF0YSA9IChhd2FpdCBmZXRjaChcImh0dHBzOi8vZGlzY29yZC5jb20vYXBpL2d1aWxkcy84NzI4NTY2ODI1Njc0NTQ3MjAvd2lkZ2V0Lmpzb25cIikudGhlbihcbiAgICAocmVzKSA9PiByZXMuanNvbigpXG4gICkpIGFzIHtcbiAgICBwcmVzZW5jZV9jb3VudDogbnVtYmVyO1xuICB9O1xuXG4gIC8vIHNldCBkaXNjb3JkIG1lbWJlciBjb3VudFxuICBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcImRpc2NvcmRcIikhLmlubmVySFRNTCA9IGRpc2NvcmREYXRhLnByZXNlbmNlX2NvdW50LnRvTG9jYWxlU3RyaW5nKCk7XG59KSgpO1xuIiwiZXhwb3J0IGRlZmF1bHQgY2xhc3MgRmFkZUluIHtcblxuICAgIHB1YmxpYyBzdGF0aWMgcnVuRmFkZUluKG9wdGlvbnM6IHtcbiAgICAgICAgaW5pdGFsRGVsYXk6IG51bWJlcjtcbiAgICAgICAgc3BhY2luZzogbnVtYmVyO1xuICAgIH0pIHtcblxuICAgICAgICAvLyBzZXQgZGVmYXVsdCBvcHRpb25zXG5cbiAgICAgICAgY29uc3QgZWxlbWVudHMgPSBkb2N1bWVudC5xdWVyeVNlbGVjdG9yQWxsKFwiI2ZhZGUtaW5cIik7XG5cbiAgICAgICAgLy8gb3JkZXIgYnkgYGRhdGEtZmFkZS1vcmRlcmAgYXR0cmlidXRlXG4gICAgICAgIGNvbnN0IG9yZGVyZWRFbGVtZW50czoge1xuICAgICAgICAgICAgZWxlbWVudDogRWxlbWVudDtcbiAgICAgICAgICAgIG9yZGVyOiBudW1iZXI7XG4gICAgICAgIH1bXSA9IEFycmF5LmZyb20oZWxlbWVudHMpLm1hcCgoZWxlbWVudCkgPT4ge1xuICAgICAgICAgICAgcmV0dXJuIHtcbiAgICAgICAgICAgICAgICBlbGVtZW50LFxuICAgICAgICAgICAgICAgIG9yZGVyOiBwYXJzZUludChlbGVtZW50LmdldEF0dHJpYnV0ZShcImRhdGEtZmFkZS1vcmRlclwiKSB8fCBcIjBcIiksXG4gICAgICAgICAgICB9XG4gICAgICAgIH0pXG5cbiAgICAgICAgLy8gZmFkZSBpblxuICAgICAgICBvcmRlcmVkRWxlbWVudHMuZm9yRWFjaCgoaXRlbSkgPT4ge1xuICAgICAgICAgICAgICAgIGl0ZW0uZWxlbWVudC5jbGFzc0xpc3QuYWRkKFwiaGlkZGVuXCIpO1xuICAgICAgICAgICAgc2V0VGltZW91dCgoKSA9PiB7XG4gICAgICAgICAgICAgICAgaXRlbS5lbGVtZW50LmNsYXNzTGlzdC5hZGQoXCJmYWRlLWluXCIpO1xuICAgICAgICAgICAgICAgIGl0ZW0uZWxlbWVudC5jbGFzc0xpc3QucmVtb3ZlKFwiaGlkZGVuXCIpO1xuICAgICAgICAgICAgfSwgaXRlbS5vcmRlciAqIG9wdGlvbnMuc3BhY2luZyArIG9wdGlvbnMuaW5pdGFsRGVsYXkpO1xuICAgICAgICB9KVxuXG4gICAgfVxuXG59IiwiLy8gVGhlIG1vZHVsZSBjYWNoZVxudmFyIF9fd2VicGFja19tb2R1bGVfY2FjaGVfXyA9IHt9O1xuXG4vLyBUaGUgcmVxdWlyZSBmdW5jdGlvblxuZnVuY3Rpb24gX193ZWJwYWNrX3JlcXVpcmVfXyhtb2R1bGVJZCkge1xuXHQvLyBDaGVjayBpZiBtb2R1bGUgaXMgaW4gY2FjaGVcblx0dmFyIGNhY2hlZE1vZHVsZSA9IF9fd2VicGFja19tb2R1bGVfY2FjaGVfX1ttb2R1bGVJZF07XG5cdGlmIChjYWNoZWRNb2R1bGUgIT09IHVuZGVmaW5lZCkge1xuXHRcdHJldHVybiBjYWNoZWRNb2R1bGUuZXhwb3J0cztcblx0fVxuXHQvLyBDcmVhdGUgYSBuZXcgbW9kdWxlIChhbmQgcHV0IGl0IGludG8gdGhlIGNhY2hlKVxuXHR2YXIgbW9kdWxlID0gX193ZWJwYWNrX21vZHVsZV9jYWNoZV9fW21vZHVsZUlkXSA9IHtcblx0XHQvLyBubyBtb2R1bGUuaWQgbmVlZGVkXG5cdFx0Ly8gbm8gbW9kdWxlLmxvYWRlZCBuZWVkZWRcblx0XHRleHBvcnRzOiB7fVxuXHR9O1xuXG5cdC8vIEV4ZWN1dGUgdGhlIG1vZHVsZSBmdW5jdGlvblxuXHRfX3dlYnBhY2tfbW9kdWxlc19fW21vZHVsZUlkXS5jYWxsKG1vZHVsZS5leHBvcnRzLCBtb2R1bGUsIG1vZHVsZS5leHBvcnRzLCBfX3dlYnBhY2tfcmVxdWlyZV9fKTtcblxuXHQvLyBSZXR1cm4gdGhlIGV4cG9ydHMgb2YgdGhlIG1vZHVsZVxuXHRyZXR1cm4gbW9kdWxlLmV4cG9ydHM7XG59XG5cbiIsIiIsIi8vIHN0YXJ0dXBcbi8vIExvYWQgZW50cnkgbW9kdWxlIGFuZCByZXR1cm4gZXhwb3J0c1xuLy8gVGhpcyBlbnRyeSBtb2R1bGUgaXMgcmVmZXJlbmNlZCBieSBvdGhlciBtb2R1bGVzIHNvIGl0IGNhbid0IGJlIGlubGluZWRcbnZhciBfX3dlYnBhY2tfZXhwb3J0c19fID0gX193ZWJwYWNrX3JlcXVpcmVfXyhcIi4vZGF0YS9jbGllbnQvdHMvaW5kZXgudHNcIik7XG4iLCIiXSwibmFtZXMiOltdLCJzb3VyY2VSb290IjoiIn0=