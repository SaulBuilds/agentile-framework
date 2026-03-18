import { IsInt, IsString, Max, Min, MinLength } from 'class-validator';

export class CreateSubmissionDto {
  @IsString()
  @MinLength(2)
  category!: string;

  @IsInt()
  @Min(1)
  @Max(5)
  estimatedBand!: number;

  @IsString()
  @MinLength(10)
  conditionDescription!: string;
}
