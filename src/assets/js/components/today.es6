window.bus = new Vue();

Vue.component('today', {
  props: ['date'],
  template: `
    <div class="today">
      <div class="columns">
        <div class="column">
          <h1 class="title is-1" v-if="workday">{{ timeWorked.hours }}:{{ timeWorked.minutes }}</h1>
          <p class="subtitle">time worked today</p>

          <article class="message">
            <div class="message-header">
              <p>Manual controls</p>
            </div>
            <div class="message-body">
              <a class="button is-black" @click="start">Start Working</a>
              &nbsp;
              <a class="button is-white" @click="stop">Stop Working</a>
            </div>
          </article>
        </div>
        <div class="column is-offset-1 is-9">
          <day-summary v-if="workday" :date="date" :stamps="workday.stamps"></day-summary>

          <ul v-if="workday" v-for="stamp in connectingStamps">
            <li>{{ stamp.event }}</li>
          </ul>
        </div>
      </div>
    </div>
    `,
  data() {
    return {
      workday: null,
      working: false,
    };
  },
  mounted() {
    this.updateWorkday();
  },
  computed: {
    connectingStamps() {
      let enter = null;
      if (this.workday === null) {
        return [];
      }

      return this.workday.stamps.filter(stamp => {
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
    },
    timeWorked() {
      let dur = moment.duration(0);
      let enter = null;
      this.workday.stamps.forEach(stamp => {
        const m = moment(stamp.time);
        if (enter && stamp.event === 'exit') {
          dur.add(m.diff(enter));
          enter = null;
        }
        
        if (stamp.event === 'enter') {
            enter = m;
        }
      });

      if (enter) {
        dur.add(moment().diff(enter));
      }

      return {
        hours: `0${dur.hours()}`.slice(-2),
        minutes: `0${dur.minutes()}`.slice(-2),
      };
    }
  },
  methods: {
    async start() {
      try {
        this.workday.stamps.push(JSON.parse(await http.get(`/api/v1/fence/${window.DUTRACK.fence}/enter`)));
      } catch(e) {
        return;
      }
    },
    async stop() {
      try {
        this.workday.stamps.push(JSON.parse(await http.get(`/api/v1/fence/${window.DUTRACK.fence}/exit`)));
      } catch(e) {
        return;
      }
    },
    async updateWorkday() {
      this.workday = JSON.parse(await http.get(`/api/v1/workday/?date=${moment().format('YYYY-MM-DD')}`));

      if (this.workday.stamps[this.workday.stamps.length - 1].event === 'enter') {
        this.working = true;
      }
    },
  },
});
