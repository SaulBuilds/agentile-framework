module.exports = {
  default: {
    format: ['@cucumber/pretty-formatter'],
    paths: ['test/bdd/features/*.feature'],
    require: ['test/bdd/stepDefinitions/*.js'],
    worldParameters: {
      pauseOnFailure: true
    }
  }
};
