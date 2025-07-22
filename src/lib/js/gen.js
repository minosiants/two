
const stream = function (gen) {
  let _gen = gen;
  const toArray = () => {
    return Array.from(_gen);
  }
  return {
    take(n) {
      const self = this;
      const g = _gen;
      function* limited() {
        let c = 0;
        for (const v of g) {
          if (c++ >= n) break;
          yield v;
        }
      }
      _gen = limited();
      return self;
    },
    toArray: toArray
  }

};

function* gen(yielding) {
  while (true) {
    yield yielding;
  }
}
function charGen() {
  const choices = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";
  return gen(() => choices.charAt(Math.floor(Math.random() * choices.length)));
}



function stringGen(len) {
  return gen(() => stream(charGen()).take(len).toArray().join());

}


export default {
  charGen,
  stringGen,
  stream

};
