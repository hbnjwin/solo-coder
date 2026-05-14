package com.linkyoyo.tobacco.dto;

import com.linkyoyo.tobacco.entity.neo4j.EnhancedStandardNode;
import com.linkyoyo.tobacco.entity.neo4j.NormativeReferenceNode;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.ArrayList;
import java.util.List;

/**
 * 标准引用关系响应
 * 包含标准文档信息和引用关系信息
 */
@Data
@AllArgsConstructor
@NoArgsConstructor
@Builder
public class StandardReferenceResponse {

    /**
     * 标准文档信息
     */
    private EnhancedStandardNode standardNode;

    /**
     * 规范性引用文件节点列表
     */
    private List<NormativeReferenceNode> normativeReferenceNodes = new ArrayList<>();

    /**
     * 引用该标准的标准文档列表（被引用关系）
     */
    private List<ReferencingStandard> referencingStandards = new ArrayList<>();

    /**
     * 该标准引用的标准文档列表（引用关系）
     */
    private List<ReferencedStandard> referencedStandards = new ArrayList<>();

    /**
     * 引用该标准的标准文档信息
     */
    @Data
    @AllArgsConstructor
    @NoArgsConstructor
    @Builder
    public static class ReferencingStandard {
        /**
         * 标准ID
         */
        private String standardId;

        /**
         * 标准号
         */
        private String standardNo;

        /**
         * 标准名称
         */
        private String standardName;

        /**
         * 标题
         */
        private String title;

        /**
         * 引用类型
         */
        private String referenceType;
    }

    /**
     * 被该标准引用的标准文档信息
     */
    @Data
    @AllArgsConstructor
    @NoArgsConstructor
    @Builder
    public static class ReferencedStandard {
        /**
         * 标准ID
         */
        private String standardId;

        /**
         * 标准号
         */
        private String standardNo;

        /**
         * 标准名称
         */
        private String standardName;

        /**
         * 标题
         */
        private String title;

        /**
         * 引用类型
         */
        private String referenceType;
    }
}
