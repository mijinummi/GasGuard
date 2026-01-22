import { Controller, Post, Get, Body, Query } from '@nestjs/common';
import { AnalyzerService } from './analyzer.service';
import { Language } from '@engine/core';


@Controller('api/analyze')
export class AnalyzerController {
  constructor(private readonly analyzerService: AnalyzerService) {}

  @Post()
  async analyze(
    @Body() dto: {
      code: string;
      filePath: string;
      language: Language;
      config?: any;
    }
  ) {
    const result = await this.analyzerService.analyzeCode(
      dto.code,
      dto.filePath,
      dto.language,
      dto.config
    );

    return {
      success: true,
      data: {
        findings: result.findings,
        summary: result.summary,
        filesAnalyzed: result.filesAnalyzed,
        analysisTime: result.analysisTime,
        totalGasSavings: result.totalEstimatedGasSavings,
      },
      errors: result.errors,
    };
  }

  @Get('languages')
  getSupportedLanguages() {
    return {
      languages: this.analyzerService.getSupportedLanguages(),
    };
  }

  @Get('rules')
  getRules(@Query('language') language?: Language) {
    return {
      rules: this.analyzerService.getRules(language),
    };
  }
}