import { ChartOptions, ChartType } from 'chart.js';
import { Label } from 'ng2-charts';

export class ChartDataClass {

  labels: Label[];
  values: number[];
  type: ChartType;
  legend: boolean;
  plugins: any[];
  colors: object[];
  options: ChartOptions;

}
