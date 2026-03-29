import React, { useEffect, useRef } from 'react';
import p5 from 'p5';

const GardenRenderer = ({ seeds }) => {
  const p5Sketch = useRef();

  useEffect(() => {
    p5Sketch.current = new p5((sketch) => {
      let canvas;

      sketch.setup = () => {
        canvas = sketch.createCanvas(800, 600);
        canvas.parent('garden-container');
        sketch.background(220);
      };

      sketch.draw = () {
        sketch.background(220);

        // Draw grid for reference
        sketch.stroke(200);
        for (let x = 0; x <= sketch.width; x += 50) {
          sketch.line(x, 0, x, sketch.height);
        }
        for (let y = 0; y <= sketch.height; y += 50) {
          sketch.line(0, y, sketch.width, y);
        }

        // Draw each seed
        seeds.forEach((seed) => {
          const { x, y, state, checkpoint, maxCheckpoint, growthScore } = seed;

          // Calculate size based on checkpoint progress (0 to 1)
          const progress = maxCheckpoint > 0 ? checkpoint / maxCheckpoint : 0;
          const baseSize = 20;
          const size = baseSize + progress * 30; // 20 to 50 pixels

          // Determine color based on state
          let color;
          switch (state) {
            case 'PLANTED':
              color = sketch.color(139, 69, 19); // brown (sprout)
              break;
            case 'GROWING':
              color = sketch.color(34, 139, 34); // forest green
              break;
            case 'READY':
              color = sketch.color(0, 128, 0); // dark green
              break;
            case 'HARVESTED':
              color = sketch.color(0, 100, 0); // very dark green
              break;
            case 'FAILED':
              color = sketch.color(139, 0, 0); // dark red
              break;
            default:
              color = sketch.color(100, 100, 100); // gray
          }

          sketch.fill(color);
          sketch.noStroke();
          sketch.ellipse(x, y, size, size);

          // Draw a simple leaf or sprout indicator
          sketch.fill(0);
          sketch.textSize(10);
          sketch.textAlign(sketch.CENTER, sketch.BOTTOM);
          sketch.text(`C: ${checkpoint}/${maxCheckpoint}`, x, y - size / 2 - 5);
        });
      };

      // Handle window resizing
      sketch.windowResized = () => {
        sketch.resizeCanvas(sketch.windowWidth, sketch.windowHeight);
      };
    }, document.getElementById('garden-container'));

    return () => {
      p5Sketch.current.remove();
    };
  }, [seeds]);

  return <div id="garden-container" />;
};

export default GardenRenderer;