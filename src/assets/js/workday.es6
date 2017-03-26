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