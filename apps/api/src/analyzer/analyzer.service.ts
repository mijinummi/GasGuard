import { Injectable, OnModuleInit, OnModuleDestroy } from '@nestjs/common';
import {
  AnalyzerRegistry,
  Language,
  AnalyzerConfig,
} from '@engine/core';

@Injectable()
export class AnalyzerService implements OnModuleInit, OnModuleDestroy {
  private registry: AnalyzerRegistry;

  constructor() {
    this.registry = new AnalyzerRegistry();
  }

  async onModuleInit() {
    await this.registry.initializeAll();
  }

  async onModuleDestroy() {
    await this.registry.disposeAll();
  }

  async analyzeCode(
    code: string,
    filePath: string,
    language: Language,
    config?: AnalyzerConfig
  ) {
    return this.registry.analyze(code, filePath, language, config);
  }

  getSupportedLanguages() {
    return this.registry.getSupportedLanguages();
  }

  getRules(language?: Language) {
    return this.registry.getAllRules(language);
  }
}