import { Test, TestingModule } from '@nestjs/testing';
import { ScreeningService } from './screening.service';
import { PrismaService } from '../prisma/prisma.service';

describe('ScreeningService', () => {
  let service: ScreeningService;
  let prisma: {
    restrictedItemRule: { findMany: jest.Mock };
  };

  beforeEach(async () => {
    prisma = {
      restrictedItemRule: { findMany: jest.fn() },
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        ScreeningService,
        { provide: PrismaService, useValue: prisma },
      ],
    }).compile();

    service = module.get<ScreeningService>(ScreeningService);
  });

  it('should allow items with no matching rules', async () => {
    prisma.restrictedItemRule.findMany.mockResolvedValue([]);

    const result = await service.screen('power tools');
    expect(result.blocked).toBe(false);
    expect(result.requiresReview).toBe(false);
  });

  it('should block items matching BLOCKED rule', async () => {
    prisma.restrictedItemRule.findMany.mockResolvedValue([
      {
        id: 'rule-1',
        categoryPattern: 'firearm*',
        ruleType: 'BLOCKED',
        description: 'Firearms and weapon parts not allowed',
        active: true,
      },
    ]);

    const result = await service.screen('firearms');
    expect(result.blocked).toBe(true);
    expect(result.reason).toBe('Firearms and weapon parts not allowed');
    expect(result.matchedRuleId).toBe('rule-1');
  });

  it('should block items matching RECALL rule with source', async () => {
    prisma.restrictedItemRule.findMany.mockResolvedValue([
      {
        id: 'rule-2',
        categoryPattern: 'recalled widget*',
        ruleType: 'RECALL',
        description: 'Widget X recall',
        source: 'CPSC-2026-001',
        active: true,
      },
    ]);

    const result = await service.screen('recalled widget model A');
    expect(result.blocked).toBe(true);
    expect(result.reason).toContain('Recall: Widget X recall');
    expect(result.reason).toContain('CPSC-2026-001');
  });

  it('should flag items matching MANUAL_REVIEW rule', async () => {
    prisma.restrictedItemRule.findMany.mockResolvedValue([
      {
        id: 'rule-3',
        categoryPattern: '*ambiguous*',
        ruleType: 'MANUAL_REVIEW',
        description: 'Requires operator classification',
        active: true,
      },
    ]);

    const result = await service.screen('some ambiguous item');
    expect(result.blocked).toBe(false);
    expect(result.requiresReview).toBe(true);
    expect(result.reason).toBe('Requires operator classification');
  });

  it('should match case-insensitively', async () => {
    prisma.restrictedItemRule.findMany.mockResolvedValue([
      {
        id: 'rule-4',
        categoryPattern: 'FIREARM*',
        ruleType: 'BLOCKED',
        description: 'Blocked',
        active: true,
      },
    ]);

    const result = await service.screen('Firearms');
    expect(result.blocked).toBe(true);
  });

  it('should support wildcard at start of pattern', async () => {
    prisma.restrictedItemRule.findMany.mockResolvedValue([
      {
        id: 'rule-5',
        categoryPattern: '*battery',
        ruleType: 'MANUAL_REVIEW',
        description: 'Battery items need review',
        active: true,
      },
    ]);

    const result = await service.screen('lithium battery');
    expect(result.requiresReview).toBe(true);
  });

  it('should support wildcard in middle of pattern', async () => {
    prisma.restrictedItemRule.findMany.mockResolvedValue([
      {
        id: 'rule-6',
        categoryPattern: 'hazardous*material',
        ruleType: 'BLOCKED',
        description: 'Hazmat blocked',
        active: true,
      },
    ]);

    const result = await service.screen('hazardous chemical material');
    expect(result.blocked).toBe(true);
  });

  it('should not match non-matching categories', async () => {
    prisma.restrictedItemRule.findMany.mockResolvedValue([
      {
        id: 'rule-7',
        categoryPattern: 'firearm*',
        ruleType: 'BLOCKED',
        description: 'Blocked',
        active: true,
      },
    ]);

    const result = await service.screen('power tools');
    expect(result.blocked).toBe(false);
    expect(result.requiresReview).toBe(false);
  });
});
