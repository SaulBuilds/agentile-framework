import { IsBoolean } from 'class-validator';

export class SubmitItemDto {
  @IsBoolean()
  declareNotStolen!: boolean;

  @IsBoolean()
  declareNotRecalled!: boolean;

  @IsBoolean()
  declareNoHazardous!: boolean;

  @IsBoolean()
  declareMeetsPackaging!: boolean;

  @IsBoolean()
  declareRightToContribute!: boolean;
}
