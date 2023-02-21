import Particle from "./bgResources/particle";

const canvas = document.createElement("canvas");
document.body.appendChild(canvas);

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

canvas.classList.add("bg");

const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

let width = canvas.width;
let height = canvas.height;

let lastFrame = Date.now();

let particles: Particle[] = [];

let mouse = {
    x: 0,
    y: 0,
};

for (let i = 0; i < 100; i++) {
  let p = new Particle({
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
