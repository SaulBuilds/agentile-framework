import { Injectable } from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';

export interface ScreeningResult {
  blocked: boolean;
  requiresReview: boolean;
  reason?: string;
  matchedRuleId?: string;
}

@Injectable()
export class ScreeningService {
  constructor(private readonly prisma: PrismaService) {}

  /**
   * Screen an item category against the restricted item rules.
   * Rules are pattern-matched against the category string.
   *
   * Rule types:
   * - BLOCKED: Auto-reject, no submission allowed
   * - MANUAL_REVIEW: Flag for operator review before intake
   * - RECALL: Auto-block with recall source logged
   */
  async screen(category: string): Promise<ScreeningResult> {
    const normalizedCategory = category.toLowerCase().trim();

    // Fetch active rules
    const rules = await this.prisma.restrictedItemRule.findMany({
      where: { active: true },
    });

    for (const rule of rules) {
      const pattern = rule.categoryPattern.toLowerCase();

      // Support wildcard patterns: "firearm*" matches "firearms", "firearm parts"
      const matches = this.matchPattern(normalizedCategory, pattern);

      if (matches) {
        if (rule.ruleType === 'BLOCKED') {
          return {
            blocked: true,
            requiresReview: false,
            reason: rule.description,
            matchedRuleId: rule.id,
          };
        }

        if (rule.ruleType === 'RECALL') {
          return {
            blocked: true,
            requiresReview: false,
            reason: `Recall: ${rule.description}${rule.source ? ` (Source: ${rule.source})` : ''}`,
            matchedRuleId: rule.id,
          };
        }

        if (rule.ruleType === 'MANUAL_REVIEW') {
          return {
            blocked: false,
            requiresReview: true,
            reason: rule.description,
            matchedRuleId: rule.id,
          };
        }
      }
    }

    return { blocked: false, requiresReview: false };
  }

  /**
   * Simple pattern matching with wildcard (*) support.
   * "firearm*" matches "firearms", "firearm parts"
   * "weapon" matches "weapon" exactly
   */
  private matchPattern(value: string, pattern: string): boolean {
    if (pattern.startsWith('*') && pattern.endsWith('*') && pattern.length > 1) {
      const inner = pattern.slice(1, -1);
      return inner === '' || value.includes(inner);
    }
    if (pattern.endsWith('*')) {
      return value.startsWith(pattern.slice(0, -1));
    }
    if (pattern.startsWith('*')) {
      return value.endsWith(pattern.slice(1));
    }
    if (pattern.includes('*')) {
      const parts = pattern.split('*');
      return parts.every((part) => value.includes(part));
    }
    return value.includes(pattern);
  }
}
