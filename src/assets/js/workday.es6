function parseStamps(stamps) {
  const retn = {
    connecting: [],
    enterExitPairs: [],
    exitEnterPairs: [],
    events: [],
    openEnterEvent: false,
  };

  let enter = null;
  retn.connecting = stamps.filter(stamp => {
    if (enter && stamp.event === 'exit') {
      enter = null;
      return true;
    }

    if (!enter && stamp.event === 'enter') {
      enter = stamp;
      return true;
    }
    return false;
  });
  retn.openEnterEvent = enter !== null;


  if (retn.connecting.length > 0) {
    [...retn.connecting].reduce((prev, cur, idx) => {
      if (prev && idx % 2 == 1) {
        retn.enterExitPairs.push([prev, cur]);
      }
    });

    let prev = null;
    [...retn.connecting].forEach((cur, idx) => {
      if (prev && idx % 2 == 0) {
        retn.exitEnterPairs.push([prev, cur]);
      }

      prev = cur;
    });
  }

  if (retn.connecting.length > 0) {
    const m = moment(retn.connecting[0].time);
    retn.events.push({
      type: 'start_work',
      from: m,
      to: m,
    });
  }
  if (retn.connecting.length > 1 && !retn.openEnterEvent) {
    const m = moment(retn.connecting[retn.connecting.length - 1].time);
    retn.events.push({
      type: 'stop_work',
      from: m,
      to: m,
    });
  }

  retn.exitEnterPairs.forEach(pair => {
    const m1 = moment(pair[0].time);
    const m2 = moment(pair[1].time);
    const diff = m2.diff(m1, 'minutes');

    if (diff < 1) {
      retn.events.push({
        type: 'tiny_break',
        from: m1,
        to: m2,
      });
    } else if (diff < 30) {
      retn.events.push({
        type: 'short_break',
        from: m1,
        to: m2,
      });
    } else {
      retn.events.push({
        type: 'long_break',
        from: m1,
        to: m2,
      });
    }
  });

  retn.events = retn.events.sort((a, b) => {
    return a.from > b.from;
  });

  return retn;
}

(async () => {
  moment.tz.setDefault('UTC');

  new Vue({
    el: '#main',
    data: {
      history: [],
      today: moment().format('YYYY-MM-DD'),
    },
    mounted() {
      const now = moment();

      for (let i = 1; i < 7; i++) {
        setTimeout(async () => {
          const date = now.subtract(1, 'days').format('YYYY-MM-DD');
          const obj = { date, workday: { stamps: [] } };

          this.history.push(obj);

          http.get(`/api/v1/workdays?date=${date}`)
            .then(data => {
              obj.workday = JSON.parse(data);
            })
            .catch(() => { });
        }, 5);
      }
    },
  });
})();