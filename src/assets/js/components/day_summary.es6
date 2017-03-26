Vue.component('day-summary', {
  template: `
    <div class="day-summary">
      <h3 class="title" v-if="daysAgo === 0">Today</h3>
      <h3 class="title" v-else-if="daysAgo === 1">Yesterday</h3>
      <h3 class="title" v-else>{{ date }}</h3>
      <p class="subtitle">time worked: {{ timeWorked.hours }}:{{ timeWorked.minutes }}</p>
    </div>
  `,
  props: {
    date: String,
    stamps: Array,
  },
  data() {
    return {
      daysAgo: 0,
    };
  },
  mounted() {
    this.daysAgo = moment().startOf('day').diff(moment(this.date).startOf('day'), 'days');
  },
  computed: {
    timeWorked() {
      if (!this.stamps) {
        return {
          hours: '00',
          minutes: '00',
        };
      }

      let dur = moment.duration(0);
      let enter = null;
      this.stamps.forEach(stamp => {
        const m = moment(stamp.time);
        if (enter && stamp.event === 'exit') {
          dur.add(m.diff(enter));
          enter = null;
        }

        if (stamp.event === 'enter') {
          enter = m;
        }
      });

      return {
        hours: `0${dur.hours()}`.slice(-2),
        minutes: `0${dur.minutes()}`.slice(-2),
      };
    }
  },
  methods: {
    timeSince(time) {
      return moment(time).from();
    }
  }
});