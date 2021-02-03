const s = (sketch) => {

    let angle = 0.3;
    let frameIter = 0;
    const maxiterations = 200;
    const frameLayer = 2;
    const colorsRed = [];
    const colorsGreen = [];
    const colorsBlue = [];

    sketch.setup = () => {
        sketch.pixelDensity(1);
        sketch.createCanvas(window.innerWidth, window.innerHeight);
        sketch.background(255);
        sketch.colorMode(sketch.HSB, 1);
        for (let n = 0; n < maxiterations; n++) {
            let hu = sketch.sqrt(n / maxiterations);
            let col = sketch.color(hu, 255, 150);
            colorsRed[n] = sketch.red(col);
            colorsGreen[n] = sketch.green(col);
            colorsBlue[n] = sketch.blue(col);
        }
        sketch.loadPixels();
    }

    sketch.draw = () => {
        let ca = sketch.cos(angle * 3.213);
        let cb = sketch.sin(angle);
        angle += 0.0002;
        frameIter++;
        let w = 5;
        let h = (w * sketch.height) / sketch.width;
        let xmin = -w / 8;
        let ymin = -h / 8;
        let xmax = xmin + w;
        let ymax = ymin + h;
        let dx = (xmax - xmin) / sketch.width / 4;
        let dy = (ymax - ymin) / sketch.height / 4;
        let y = ymin;
        for (let j = frameIter % frameLayer; j < sketch.height / frameLayer; j++) {
            let x = xmin;
            for (let i = 0; i < sketch.width; i++) {
                let a = x;
                let b = y;
                let n = 0;
                while (n < maxiterations) {
                    let aa = a * a;
                    let bb = b * b;
                    if (aa + bb > 4.0) {
                        break;
                    }
                    let twoab = 2.0 * a * b;
                    a = aa - bb + ca;
                    b = twoab + cb;
                    n++;
                }
                let pix = (i + ((j * frameLayer - frameIter % frameLayer) * sketch.width)) * 4;
                if (n == maxiterations) {
                    sketch.pixels[pix + 0] = 0;
                    sketch.pixels[pix + 1] = 0;
                    sketch.pixels[pix + 2] = 0;
                } else {
                    sketch.pixels[pix + 0] = colorsRed[n];
                    sketch.pixels[pix + 1] = colorsRed[n];
                    sketch.pixels[pix + 2] = colorsRed[n];
                }
                x += dx;
            }
            y += dy * frameLayer;
        }
        sketch.updatePixels();
        sketch.colorMode(sketch.RGB, 255);
        sketch.fill(153, 140, 54 / 2, 200);
        sketch.rect(0, 0, sketch.width, sketch.height);
    }
}

let myp5 = new p5(s);