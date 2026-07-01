import chalk from 'chalk';

const ALIEN_ART = `
${chalk.green('    ╔══════════════════╗')}
${chalk.green('    ║')}  ${chalk.cyan('⣀⣤⣴⣶⣶⣴⣤⣀')}  ${chalk.green('║')}
${chalk.green('    ║')} ${chalk.cyan('⣴⣿')}${chalk.white('⠿⠟⠛⠛⠻⠿')}${chalk.cyan('⣿⣴')} ${chalk.green('║')}
${chalk.green('    ║')}${chalk.cyan('⣾⣿')}${chalk.white('⠋')}  ${chalk.yellowBright('◉')}  ${chalk.yellowBright('◉')}  ${chalk.white('⠙')}${chalk.cyan('⣿⣾')}${chalk.green('║')}
${chalk.green('    ║')}${chalk.cyan('⣿⣿')}    ${chalk.magenta('▽')}    ${chalk.cyan('⣿⣿')}${chalk.green('║')}
${chalk.green('    ║')} ${chalk.cyan('⠻⣿⣄')}  ${chalk.red('⌣')}  ${chalk.cyan('⣠⣿⠟')} ${chalk.green('║')}
${chalk.green('    ║')}  ${chalk.cyan('⠙⠿⣿⣶⣶⣿⠿⠋')}  ${chalk.green('║')}
${chalk.green('    ╚══════════════════╝')}
`;

const TITLE = chalk.bold.cyan('  C O M P A N I O N');
const SUBTITLE = chalk.dim('  AI Agent • v0.1.0');

export function showSplash(): void {
  console.log(ALIEN_ART);
  console.log(TITLE);
  console.log(SUBTITLE);
  console.log(chalk.dim('  ─────────────────────'));
  console.log('');
}
