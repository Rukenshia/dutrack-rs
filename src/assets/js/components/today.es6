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
              <a class="button is-black" @click="stop" v-if="parsed.openEnterEvent || false">Stop Working</a>
              <a class="button is-black" @click="start" v-else>Start Working</a>
            </div>
          </article>
        </div>
        <div class="column is-offset-1 is-9">
          <day-summary :now="now" :date="date" :stamps="workday.stamps"></day-summary>

          <template v-if="parsed !== null">
          <template v-for="event in parsed.events">
          <div class="columns">
            <div class="column">
              <event :now="now" :type="event.type" :from="event.from" :until="event.to"></event>
            </div>
          </div>
          </template>
          </template>
        </div>
      </div>
    </div>
    `,
  data() {
    return {
      workday: {
        stamps: [],
      },
      parsed: {
        connecting: [],
        events: [],
        openEnterEvent: false,
      },
      intv: null,
      now: moment(),
    };
  },
  mounted() {
    this.updateWorkday();

  },
  watch: {
    workday(wd) {
      this.parsed = window.parseStamps(wd.stamps);
      this.parsed.events.reverse();

      if (this.parsed.openEnterEvent) {  
        this.intv = setInterval(() => this.updateNow(), 2000);
      } else {
        clearInterval(this.intv);
      }
    }
  },
  computed: {
    connectingStamps() {
      if (this.workday === null) {
        return [];
      }

      return window.parseStamps(this.workday.stamps).connecting;
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
        dur.add(this.now.diff(enter));
      }

      return {
        hours: `0${dur.hours()}`.slice(-2),
        minutes: `0${dur.minutes()}`.slice(-2),
      };
    }
  },
  methods: {
    updateNow() {
      this.now = moment();
    },
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
      try {
        this.workday = JSON.parse(await http.get(`/api/v1/workdays/?date=${moment().format('YYYY-MM-DD')}`));
      } catch(e) {
        this.workday = {
          stamps: [],
        };
      }
    },
  },
});
