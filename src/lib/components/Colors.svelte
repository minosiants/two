<script>
  function hexToHsl(hex) {
    // Remove "#" if present
    hex = hex.replace(/^#/, "");

    // Convert shorthand hex (e.g. "#f00") to full (e.g. "#ff0000")
    if (hex.length === 3) {
      hex = hex
        .split("")
        .map((c) => c + c)
        .join("");
    }

    const r = parseInt(hex.slice(0, 2), 16);
    const g = parseInt(hex.slice(2, 4), 16);
    const b = parseInt(hex.slice(4, 6), 16);

    // Convert RGB to range [0,1]
    const rNorm = r / 255;
    const gNorm = g / 255;
    const bNorm = b / 255;

    const max = Math.max(rNorm, gNorm, bNorm);
    const min = Math.min(rNorm, gNorm, bNorm);
    let h, s, l;

    l = (max + min) / 2;

    if (max === min) {
      h = s = 0; // achromatic
    } else {
      const d = max - min;
      s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
      switch (max) {
        case rNorm:
          h = (gNorm - bNorm) / d + (gNorm < bNorm ? 6 : 0);
          break;
        case gNorm:
          h = (bNorm - rNorm) / d + 2;
          break;
        case bNorm:
          h = (rNorm - gNorm) / d + 4;
          break;
      }
      h /= 6;
    }
    h = Math.round(h * 360);
    s = Math.round(s * 100);
    l = Math.round(l * 100);
    return {
      h,
      s,
      l,
    };
  }
  function* sequenceGenerator(start, end, step) {
    for (let i = start; i <= end; i += step) {
      yield i;
    }
  }
  function clamp(value, min, max) {
    return Math.max(min, Math.min(max, value));
  }
  function range(color) {
    const hsl = hexToHsl(color);
    const start = clamp(hsl.s - 25, 0, 100);
    const end = clamp(hsl.s + 25, 0, 100);
    const res = [...sequenceGenerator(start, end, 5)].map((v) => ({
      ...hsl,
      s: v,
    }));
    console.log(hsl);
    console.log(res);
    return res;
  }
  function hslStr(v) {
    return `hsl(${v.h}, ${v.s}%, ${v.l}%)`;
  }

  const color1 = "#caa885"; /*2311cp*/
  const color2 = "#96694a"; /*1535up*/
  const color3 = "#b86757"; /*18-1535tpg*/
  const color4 = "#2a554c"; /*2566cp*/
  const color5 = "#6a282c"; /*19-1535tcx*/
  const colors = [color1, color2, color3, color4, color5];

  const cssColorTemplate = (color, pantone) => {
    const hsl = hslStr(hexToHsl(color));
    const colorMainVar = `--color-${pantone}:${hsl}; /* ${color} */`;
    const rangeColors = range(color)
      .map((hsl, i) => {
        return `--color-${pantone}-${i}:${hslStr(hsl)};`;
      })
      .join("<br/>");
    return `
      ${colorMainVar}<br/>
      ${rangeColors}<br/>`;
  };
</script>

<section class="center">
  <div class="stack">
    <div>{@html cssColorTemplate(color1, "2311CP")}</div>
    <div>{@html cssColorTemplate(color2, "1535UP")}</div>
    <div>{@html cssColorTemplate(color3, "18-1535TPG")}</div>
    <div>{@html cssColorTemplate(color4, "2566CP")}</div>
    <div>{@html cssColorTemplate(color5, "19-1535tcx")}</div>
  </div>
  <ul class="stack box">
    {#each colors as color}
      <li class="box stack">
        <div class="color-box" style="background-color: {color};">
          <span>{hslStr(hexToHsl(color))}</span>
        </div>
        <div class="cluster">
          {#each range(color) as s}
            <div style="background-color: {hslStr(s)};">
              {hslStr(s)}
            </div>
            <div>
              <h1 style="color: {hslStr(s)}">This is {hslStr(s)}</h1>
              <h2 style="color: {hslStr(s)}">This is {hslStr(s)}</h2>
              <h3 style="color: {hslStr(s)}">This is {hslStr(s)}</h3>
              <span style="color: {hslStr(s)}"> This is {hslStr(s)}</span>
            </div>
          {/each}
        </div>
      </li>
    {/each}
  </ul>
</section>

<style>
  section {
    --color-1: #caa885; /*2311cp*/
    --color-2: #96694a; /*1535up*/
    --color-3: #b86757; /*18-1535tpg*/
    --color-4: #2a554c; /*2566cp*/
    --color-5: #6a282c; /*19-1535tcx*/
  }
  .stack {
    align-items: stretch;
    flex-wrap: nowrap;
  }
  .color-box {
    height: var(--s3);
  }
</style>
