export default class Particle {
  public position: Position;
  public velocity: Position = {
    x: 0,
    y: 0,
  };

  constructor(position: Position) {
    this.position = position;
  }

  public setVelocity(v: Position) {
    this.velocity = v;
  }

  public move(
    delta: number,
    mode: "bounce" | "wrap" = "wrap",
    bounds?: {
      x: number;
      y: number;
      width: number;
      height: number;
    }
  ) {
    this.position.x += this.velocity.x * delta;
    this.position.y += this.velocity.y * delta;

    if (!bounds) bounds = { x: 0, y: 0, width: window.innerWidth, height: window.innerHeight };

    if (mode === "bounce") {
      if (this.position.x < bounds.x) {
        this.position.x = bounds.x;
        this.velocity.x *= -1;
      } else if (this.position.x > bounds.x + bounds.width) {
        this.position.x = bounds.x + bounds.width;
        this.velocity.x *= -1;
      }

      if (this.position.y < bounds.y) {
        this.position.y = bounds.y;
        this.velocity.y *= -1;
      } else if (this.position.y > bounds.y + bounds.height) {
        this.position.y = bounds.y + bounds.height;
        this.velocity.y *= -1;
      }
    }

    if (mode === "wrap") {
      if (this.position.x < bounds.x) {
        this.position.x = bounds.x + bounds.width;
      } else if (this.position.x > bounds.x + bounds.width) {
        this.position.x = bounds.x;
      }

      if (this.position.y < bounds.y) {
        this.position.y = bounds.y + bounds.height;
      } else if (this.position.y > bounds.y + bounds.height) {
        this.position.y = bounds.y;
      }
    }
  }

  public draw(ctx: CanvasRenderingContext2D, mouse: Position) {
    const distance = Math.sqrt(
      Math.pow(this.position.x - mouse.x, 2) + Math.pow(this.position.y - mouse.y, 2)
    );

    const size = Math.max(10, 100 - distance) / 10;

    ctx.fillStyle = "#fff";
    
    ctx.beginPath();
    ctx.arc(this.position.x, this.position.y, size, 0, Math.PI * 2);
    ctx.fill();
    
  }
}

